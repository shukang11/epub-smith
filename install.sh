#!/bin/bash

set -e

# Function to get the latest release version from GitHub API
get_latest_version() {
    echo "Fetching latest version from GitHub API..."
    local latest_version
    latest_version=$(curl -s "https://api.github.com/repos/shukang11/epub-smith/releases/latest" | grep -o '"tag_name": "[^\"]*' | cut -d '"' -f 4)
    if [ -z "$latest_version" ]; then
        echo "Failed to fetch latest version. Using default version v0.2.0"
        latest_version="v0.2.0"
    fi
    echo "Latest version: $latest_version"
    echo "$latest_version"
}

# Check for help option
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "========================================="
    echo "EpubSmith Installer"
    echo "========================================="
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --help, -h              Show this help message"
    echo "  --version VERSION       Specify the version to install (default: latest)"
    echo "  --download-only         Only download the release, don't install"
    echo ""
    echo "Examples:"
    echo "  $0                      Install the latest version"
    echo "  $0 --version v0.2.0     Install a specific version"
    echo "  $0 --download-only      Download but don't install"
    echo ""
    exit 0
fi

# Parse command line options
VERSION=""
DOWNLOAD_ONLY=false

for arg in "$@"; do
    case "$arg" in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --download-only)
            DOWNLOAD_ONLY=true
            shift
            ;;
        *)
            echo "Unknown option: $arg"
            echo "Run '$0 --help' for usage information"
            exit 1
            ;;
    esac
done

# If no version specified, get the latest version
if [ -z "$VERSION" ]; then
    VERSION=$(get_latest_version)
fi

echo "========================================="
echo "EpubSmith Installer"
echo "========================================="

echo ""
echo "Detecting system information..."

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)     OS="linux" ;;
    Darwin*)    OS="darwin" ;;
    CYGWIN*)    OS="windows" ;;
    MINGW*)     OS="windows" ;;
    *)          echo "Unsupported OS: $OS" ; exit 1 ;;
esac

echo "OS: $OS"

# Detect architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64*)    ARCH="amd64" ;;
    arm64*)     ARCH="arm64" ;;
    aarch64*)   ARCH="arm64" ;;
    *)          echo "Unsupported architecture: $ARCH" ; exit 1 ;;
esac

echo "Architecture: $ARCH"

echo ""
echo "Determining download URL..."

# Set variables
PROJECT="shukang11/epub-smith"
VERSION="v0.2.0"
BIN_NAME="epub-smith"
SHORT_NAME="epbs"

# Determine file extension and download URL
if [ "$OS" = "windows" ]; then
    EXT="zip"
    DOWNLOAD_URL="https://github.com/$PROJECT/releases/download/$VERSION/$BIN_NAME-$OS-$ARCH.$EXT"
else
    EXT="tar.gz"
    DOWNLOAD_URL="https://github.com/$PROJECT/releases/download/$VERSION/$BIN_NAME-$OS-$ARCH.$EXT"
fi

echo "Download URL: $DOWNLOAD_URL"

echo ""
echo "Downloading and installing..."

# Create temporary directory
TEMP_DIR=$(mktemp -d)
echo "Using temporary directory: $TEMP_DIR"

# Download the release
echo "Downloading $BIN_NAME $VERSION..."
if curl -L -o "$TEMP_DIR/release.$EXT" "$DOWNLOAD_URL"; then
    echo "✓ Download completed successfully"
else
    echo "✗ Failed to download release"
    exit 1
fi

# Extract the release
echo "Extracting release..."
cd "$TEMP_DIR"
if [ "$EXT" = "tar.gz" ]; then
    if tar -xzf "release.$EXT"; then
        echo "✓ Extraction completed successfully"
    else
        echo "✗ Failed to extract release"
        exit 1
    fi
else
    if unzip "release.$EXT"; then
        echo "✓ Extraction completed successfully"
    else
        echo "✗ Failed to extract release"
        exit 1
    fi
fi

# Determine installation directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
    echo "Installing to system directory: $INSTALL_DIR"
else
    INSTALL_DIR="$HOME/.local/bin"
    echo "Installing to user directory: $INSTALL_DIR"
    # Create the directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    # Add to PATH if not already present
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        echo "Adding $INSTALL_DIR to PATH in your shell configuration..."
        if [ -f "$HOME/.bashrc" ]; then
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.bashrc"
            echo "✓ Added to ~/.bashrc"
        fi
        if [ -f "$HOME/.zshrc" ]; then
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.zshrc"
            echo "✓ Added to ~/.zshrc"
        fi
        if [ -f "$HOME/.profile" ]; then
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.profile"
            echo "✓ Added to ~/.profile"
        fi
        echo "⚠️  You may need to restart your shell or run 'source ~/.bashrc' (or equivalent) to update your PATH"
    fi
fi

# Install the binaries
echo "Installing binaries..."
if [ -f "$TEMP_DIR/$BIN_NAME-$OS-$ARCH" ]; then
    cp "$TEMP_DIR/$BIN_NAME-$OS-$ARCH" "$INSTALL_DIR/$BIN_NAME"
    cp "$TEMP_DIR/$SHORT_NAME-$OS-$ARCH" "$INSTALL_DIR/$SHORT_NAME"
    chmod +x "$INSTALL_DIR/$BIN_NAME"
    chmod +x "$INSTALL_DIR/$SHORT_NAME"
    echo "✓ $BIN_NAME installed successfully at $INSTALL_DIR/$BIN_NAME"
    echo "✓ $SHORT_NAME installed successfully at $INSTALL_DIR/$SHORT_NAME"
elif [ -f "$TEMP_DIR/$BIN_NAME.exe" ]; then
    cp "$TEMP_DIR/$BIN_NAME.exe" "$INSTALL_DIR/$BIN_NAME.exe"
    cp "$TEMP_DIR/$SHORT_NAME.exe" "$INSTALL_DIR/$SHORT_NAME.exe"
    chmod +x "$INSTALL_DIR/$BIN_NAME.exe"
    chmod +x "$INSTALL_DIR/$SHORT_NAME.exe"
    echo "✓ $BIN_NAME.exe installed successfully at $INSTALL_DIR/$BIN_NAME.exe"
    echo "✓ $SHORT_NAME.exe installed successfully at $INSTALL_DIR/$SHORT_NAME.exe"
else
    echo "✗ Failed to find binary files in the release"
    echo "Contents of $TEMP_DIR:"
    ls -la "$TEMP_DIR"
    exit 1
fi

# Clean up
echo ""
echo "Cleaning up..."
rm -rf "$TEMP_DIR"
echo "✓ Temporary files removed"

echo ""
echo "========================================="
echo "Installation completed successfully!"
echo "========================================="
echo ""
echo "You can now use EpubSmith with either command:"
echo "  $BIN_NAME --help"
echo "  $SHORT_NAME --help"
echo ""
echo "To verify the installation, run:"
echo "  $BIN_NAME --version"
echo ""
