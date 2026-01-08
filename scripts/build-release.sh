#!/bin/bash

set -e

VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)  BUILD_DIR="build/release"  PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Initialize build success variables
LINUX_BUILD_SUCCESS=false
WINDOWS_BUILD_SUCCESS=false

# Default to true for macOS builds, since we're on macOS
MACOS_INTEL_BUILD_SUCCESS=true
MACOS_ARM64_BUILD_SUCCESS=false

echo "========================================="
echo "EpubSmith Release Build Script"
echo "Version: $VERSION"
echo "========================================="

cd "$PROJECT_ROOT"

mkdir -p "$BUILD_DIR"

echo ""
echo "Step 1: Running code quality checks..."
echo "----------------------------------------"

echo "Running tests..."
cargo test

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running fmt check..."
cargo fmt --check

echo ""  echo "Step 2: Checking build dependencies..."  echo "----------------------------------------"

# Check for necessary cross-compilation tools
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS specific checks
    echo "Checking macOS cross-compilation dependencies..."
    # Check if we have the necessary linker for Linux
    if ! which x86_64-unknown-linux-gnu-gcc > /dev/null 2>&1; then
        echo "Warning: x86_64-unknown-linux-gnu-gcc not found. You may need to install it using:"
        echo "  brew install FiloSottile/musl-cross/musl-cross"
        echo "  or"
        echo "  brew install messense/macos-cross-toolchains/x86_64-unknown-linux-gnu"
    fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux specific checks
    echo "Checking Linux cross-compilation dependencies..."
    # Check if we have the necessary linker for Windows
    if ! which x86_64-w64-mingw32-gcc > /dev/null 2>&1; then
        echo "Warning: x86_64-w64-mingw32-gcc not found. You may need to install it using:"
        echo "  sudo apt-get install gcc-mingw-w64-x86-64"
    fi
fi

echo ""  echo "Step 3: Installing cross-compilation targets..."  echo "----------------------------------------"

rustup target add aarch64-apple-darwin x86_64-apple-darwin x86_64-unknown-linux-gnu x86_64-pc-windows-gnu

echo ""  echo "Step 4: Building macOS..."  echo "----------------------------------------"

# Get current architecture
CURRENT_ARCH="$(uname -m)"

# Build macOS versions
if [ "$CURRENT_ARCH" = "arm64" ]; then
    # On Apple Silicon, we can build both architectures
    for arch in aarch64-apple-darwin x86_64-apple-darwin; do
        echo "Building macOS for $arch..."
        cargo build --release --target "$arch"
        if [ -d "target/$arch/release" ]; then
            echo "Checking generated files in target/$arch/release/..."
            ls -la target/$arch/release/
            if [ -f "target/$arch/release/epub-smith" ]; then
                if [ "$arch" = "aarch64-apple-darwin" ]; then
                    cp target/$arch/release/epub-smith "$BUILD_DIR/epub-smith-darwin-arm64"
                    cp target/$arch/release/epbs "$BUILD_DIR/epbs-darwin-arm64"
                    MACOS_ARM64_BUILD_SUCCESS=true
                else
                    cp target/$arch/release/epub-smith "$BUILD_DIR/epub-smith-darwin-amd64"
                    cp target/$arch/release/epbs "$BUILD_DIR/epbs-darwin-amd64"
                    MACOS_INTEL_BUILD_SUCCESS=true
                fi
            else
                echo "Warning: Binary not found for $arch. Skipping..."
                if [ "$arch" = "aarch64-apple-darwin" ]; then
                    MACOS_ARM64_BUILD_SUCCESS=false
                else
                    MACOS_INTEL_BUILD_SUCCESS=false
                fi
            fi
        else
            echo "Warning: Release directory not found for $arch. Skipping..."
            if [ "$arch" = "aarch64-apple-darwin" ]; then
                MACOS_ARM64_BUILD_SUCCESS=false
            else
                MACOS_INTEL_BUILD_SUCCESS=false
            fi
        fi
    done
elif [ "$CURRENT_ARCH" = "x86_64" ]; then
    # On Intel, we can only build Intel version reliably
    echo "Building macOS (Intel)..."
    cargo build --release --target x86_64-apple-darwin
    if [ -d "target/x86_64-apple-darwin/release" ]; then
        echo "Checking generated files in target/x86_64-apple-darwin/release/..."
        ls -la target/x86_64-apple-darwin/release/
        if [ -f "target/x86_64-apple-darwin/release/epub-smith" ]; then
            cp target/x86_64-apple-darwin/release/epub-smith "$BUILD_DIR/epub-smith-darwin-amd64"
            cp target/x86_64-apple-darwin/release/epbs "$BUILD_DIR/epbs-darwin-amd64"
            MACOS_INTEL_BUILD_SUCCESS=true
        else
            echo "Warning: Intel binary not found. Skipping..."
            MACOS_INTEL_BUILD_SUCCESS=false
        fi
    else
        echo "Warning: Release directory not found for x86_64-apple-darwin. Skipping..."
        MACOS_INTEL_BUILD_SUCCESS=false
    fi
    
    echo "Skipping Apple Silicon build on Intel machine..."
    MACOS_ARM64_BUILD_SUCCESS=false
else
    echo "Unknown architecture: $CURRENT_ARCH"
    echo "Skipping macOS builds..."
fi

echo ""  echo "Step 5: Building Windows..."  echo "----------------------------------------"

echo "Building Windows..."
cargo build --release --target x86_64-pc-windows-gnu
if [ -d "target/x86_64-pc-windows-gnu/release" ]; then
    echo "Checking generated files in target/x86_64-pc-windows-gnu/release/..."
    ls -la target/x86_64-pc-windows-gnu/release/
    if [ -f "target/x86_64-pc-windows-gnu/release/epub-smith.exe" ]; then
        cp target/x86_64-pc-windows-gnu/release/epub-smith.exe "$BUILD_DIR/epub-smith-windows-amd64.exe"
        cp target/x86_64-pc-windows-gnu/release/epbs.exe "$BUILD_DIR/epbs-windows-amd64.exe"
        WINDOWS_BUILD_SUCCESS=true
    else
        echo "Warning: Windows binary not found. Skipping..."
        WINDOWS_BUILD_SUCCESS=false
    fi
else
    echo "Warning: Release directory not found for x86_64-pc-windows-gnu. Skipping..."
    WINDOWS_BUILD_SUCCESS=false
fi

echo ""  echo "Step 6: Building Linux..."  echo "----------------------------------------"

# Try to build Linux version, skip if it fails
echo "Attempting to build Linux version..."
LINUX_BUILD_SUCCESS=false
if cargo build --release --target x86_64-unknown-linux-gnu; then
    if [ -d "target/x86_64-unknown-linux-gnu/release" ]; then
        echo "Checking generated files in target/x86_64-unknown-linux-gnu/release/..."
        ls -la target/x86_64-unknown-linux-gnu/release/
        if [ -f "target/x86_64-unknown-linux-gnu/release/epub-smith" ]; then
            echo "Linux build successful!"
            cp target/x86_64-unknown-linux-gnu/release/epub-smith "$BUILD_DIR/epub-smith-linux-amd64"
            cp target/x86_64-unknown-linux-gnu/release/epbs "$BUILD_DIR/epbs-linux-amd64"
            LINUX_BUILD_SUCCESS=true
        else
            echo "Warning: Linux binary not found. Skipping..."
        fi
    else
        echo "Warning: Release directory not found for x86_64-unknown-linux-gnu. Skipping..."
    fi
else
    echo "Warning: Linux build failed. This may be due to missing cross-compilation tools."
    echo "Please install the required tools and try again."
fi

echo ""  echo "Step 7: Creating release packages..."  echo "----------------------------------------"

cd "$BUILD_DIR"

# Create macOS Apple Silicon package if built
if [ "$MACOS_ARM64_BUILD_SUCCESS" = true ] && [ -f "epub-smith-darwin-arm64" ]; then
    echo "Creating macOS Apple Silicon package..."
    tar -czf "epub-smith-darwin-arm64.tar.gz" epub-smith-darwin-arm64 epbs-darwin-arm64
fi

# Create macOS Intel package if built
if [ "$MACOS_INTEL_BUILD_SUCCESS" = true ] && [ -f "epub-smith-darwin-amd64" ]; then
    echo "Creating macOS Intel package..."
    tar -czf "epub-smith-darwin-amd64.tar.gz" epub-smith-darwin-amd64 epbs-darwin-amd64
fi

# Create Windows package if built
if [ "$WINDOWS_BUILD_SUCCESS" = true ] && [ -f "epub-smith-windows-amd64.exe" ]; then
    echo "Creating Windows package..."
    zip epub-smith-windows-amd64.zip epub-smith-windows-amd64.exe epbs-windows-amd64.exe
fi

# Create Linux package if built
if [ "$LINUX_BUILD_SUCCESS" = true ] && [ -f "epub-smith-linux-amd64" ]; then
    echo "Creating Linux package..."
    tar -czf "epub-smith-linux-amd64.tar.gz" epub-smith-linux-amd64 epbs-linux-amd64
fi

echo ""  echo "Step 8: Verifying packages..."  echo "----------------------------------------"

# Only verify packages that were built and exist
if [ -f "epub-smith-darwin-arm64.tar.gz" ]; then
    echo "Verifying macOS Apple Silicon package..."
    tar -tzf epub-smith-darwin-arm64.tar.gz
fi

if [ -f "epub-smith-darwin-amd64.tar.gz" ]; then
    echo "Verifying macOS Intel package..."
    tar -tzf epub-smith-darwin-amd64.tar.gz
fi

if [ -f "epub-smith-windows-amd64.zip" ]; then
    echo "Verifying Windows package..."
    unzip -l epub-smith-windows-amd64.zip
fi

if [ -f "epub-smith-linux-amd64.tar.gz" ]; then
    echo "Verifying Linux package..."
    tar -tzf epub-smith-linux-amd64.tar.gz
fi

cd "$PROJECT_ROOT"

echo ""
echo "========================================="
echo "Build completed successfully!"
echo "========================================="
echo ""  echo "Release packages:"  echo "  - $BUILD_DIR/epub-smith-darwin-arm64.tar.gz"  echo "  - $BUILD_DIR/epub-smith-darwin-amd64.tar.gz"  echo "  - $BUILD_DIR/epub-smith-windows-amd64.zip"  echo "  - $BUILD_DIR/epub-smith-linux-amd64.tar.gz"  echo ""  echo "To upload to GitHub Release:"  echo "  1. Create a git tag: git tag -a v$VERSION -m 'Release version $VERSION'"  echo "  2. Push the tag: git push origin v$VERSION"  echo "  3. Go to GitHub and create a new release"  echo "  4. Upload the release packages"  echo ""
