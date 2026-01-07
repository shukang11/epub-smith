# EpubSmith 发布技术指导

本文档提供了 EpubSmith 项目预编译二进制文件的发布步骤，用于手动上传到 GitHub Release。

## 1. 发布前准备

### 1.1 代码质量检查
```bash
# 运行所有测试
cargo test

# 运行 Clippy 静态分析
cargo clippy --all-targets --all-features -- -D warnings

# 运行代码格式化检查
cargo fmt --check
```

### 1.2 依赖安全检查
```bash
# 安装 cargo-audit（如果未安装）
cargo install cargo-audit

# 运行依赖安全扫描
cargo audit

# 锁定依赖版本
cargo update --locked
```

### 1.3 版本号更新
1. 编辑 `Cargo.toml` 文件，修改 `version` 字段
2. 遵循语义化版本规范：MAJOR.MINOR.PATCH
3. 例如：`version = "0.1.0"` → `version = "0.2.0"`

### 1.4 测试发布
```bash
cargo publish --dry-run
```

## 2. 预编译二进制文件生成

### 2.1 环境准备

#### 2.1.1 安装交叉编译工具
```bash
# 安装 cross 工具用于交叉编译
cargo install cross
```

#### 2.1.2 安装目标工具链
```bash
# 安装 macOS (Apple Silicon) 工具链
rustup target add aarch64-apple-darwin

# 安装 macOS (Intel) 工具链
rustup target add x86_64-apple-darwin

# 安装 Linux 工具链
rustup target add x86_64-unknown-linux-gnu

# 安装 Windows 工具链
rustup target add x86_64-pc-windows-gnu
```

### 2.2 编译各平台二进制文件

#### 2.2.1 创建构建目录
```bash
mkdir -p build/release
```

#### 2.2.2 编译 macOS 二进制

##### Apple Silicon (M1/M2)
```bash
cross build --release --target aarch64-apple-darwin

# 复制二进制文件到构建目录
cp target/aarch64-apple-darwin/release/epub-smith build/release/epub-smith-darwin-arm64
cp target/aarch64-apple-darwin/release/epbs build/release/epbs-darwin-arm64
```

##### Intel (x86_64)
```bash
cross build --release --target x86_64-apple-darwin

# 复制二进制文件到构建目录
cp target/x86_64-apple-darwin/release/epub-smith build/release/epub-smith-darwin-amd64
cp target/x86_64-apple-darwin/release/epbs build/release/epbs-darwin-amd64
```

#### 2.2.3 编译 Linux 二进制
```bash
cross build --release --target x86_64-unknown-linux-gnu

# 复制二进制文件到构建目录
cp target/x86_64-unknown-linux-gnu/release/epub-smith build/release/epub-smith-linux-amd64
cp target/x86_64-unknown-linux-gnu/release/epbs build/release/epbs-linux-amd64
```

#### 2.2.4 编译 Windows 二进制
```bash
cross build --release --target x86_64-pc-windows-gnu

# 复制二进制文件到构建目录
cp target/x86_64-pc-windows-gnu/release/epub-smith.exe build/release/epub-smith-windows-amd64.exe
cp target/x86_64-pc-windows-gnu/release/epbs.exe build/release/epbs-windows-amd64.exe
```

### 2.3 创建压缩包

#### 2.3.1 进入构建目录
```bash
cd build/release
```

#### 2.3.2 为各平台创建压缩包

##### macOS
```bash
# Apple Silicon
tar -czf epub-smith-darwin-arm64.tar.gz epub-smith-darwin-arm64 epbs-darwin-arm64

# Intel
tar -czf epub-smith-darwin-amd64.tar.gz epub-smith-darwin-amd64 epbs-darwin-amd64
```

##### Linux
```bash
tar -czf epub-smith-linux-amd64.tar.gz epub-smith-linux-amd64 epbs-linux-amd64
```

##### Windows
```bash
zip epub-smith-windows-amd64.zip epub-smith-windows-amd64.exe epbs-windows-amd64.exe
```

#### 2.3.3 验证压缩包
```bash
# 验证 macOS Apple Silicon 压缩包
tar -tzf epub-smith-darwin-arm64.tar.gz

# 验证 macOS Intel 压缩包
tar -tzf epub-smith-darwin-amd64.tar.gz

# 验证 Linux 压缩包
tar -tzf epub-smith-linux-amd64.tar.gz

# 验证 Windows 压缩包
unzip -l epub-smith-windows-amd64.zip
```

## 3. GitHub Release 创建与上传

### 3.1 创建 Git 标签
```bash
# 回到项目根目录
cd ../../..

# 创建带注释的标签
git tag -a v0.1.0 -m "Release version 0.1.0"  # 替换为实际版本号

# 推送标签到远程仓库
git push origin v0.1.0  # 替换为实际版本号
```

### 3.2 在 GitHub 上创建 Release

1. 登录 GitHub，进入项目页面
2. 点击顶部导航栏的 "Releases" 或 "Tags"
3. 点击 "Draft a new release"
4. 在 "Choose a tag" 下拉菜单中选择刚刚创建的标签
5. 填写 "Release title"（建议使用 "Release vX.Y.Z" 格式）
6. 填写 "Describe this release"（可包含更新日志、新功能等）
7. 滚动到 "Attach binaries by dropping them here or selecting them"
8. 上传以下文件：
   - `build/release/epub-smith-darwin-arm64.tar.gz`
   - `build/release/epub-smith-darwin-amd64.tar.gz`
   - `build/release/epub-smith-linux-amd64.tar.gz`
   - `build/release/epub-smith-windows-amd64.zip`
9. 选择是否将此 Release 设为 "Latest release"
10. 点击 "Publish release"

## 4. 发布后验证

### 4.1 下载测试
1. 从 GitHub Release 下载一个二进制文件
2. 解压文件
3. 运行版本检查：
   ```bash
   ./epub-smith --version
   ./epbs --version
   ```

### 4.2 功能测试
1. 创建一个简单的测试文本文件
2. 使用二进制文件进行转换：
   ```bash
   ./epub-smith test.txt
   ```
3. 验证生成的 EPUB 文件是否正常

## 5. 常见问题与解决方案

### 5.1 交叉编译失败
- 确保安装了正确的目标工具链：`rustup target add TARGET`
- 使用 cross 工具：`cross build --target TARGET`
- 检查依赖是否支持目标平台

### 5.2 压缩包创建失败
- 确保安装了 tar 和 zip 工具：
  ```bash
  # macOS
  brew install tar zip
  
  # Ubuntu/Debian
  sudo apt-get install tar zip
  
  # CentOS/Fedora
  sudo yum install tar zip
  ```

### 5.3 GitHub 上传失败
- 确保文件大小不超过 GitHub 的限制（每个文件不超过 2GB）
- 检查网络连接是否稳定
- 尝试使用 GitHub CLI 上传：
  ```bash
  # 安装 GitHub CLI（如果未安装）
  brew install gh  # macOS
  # 或
  sudo apt-get install gh  # Ubuntu/Debian
  
  # 登录 GitHub CLI
  gh auth login
  
  # 上传发布文件
  gh release upload v0.1.0 build/release/*  # 替换为实际版本号
  ```

## 6. 发布清单

在发布过程中，确保完成以下所有步骤：

- [ ] 运行所有测试
- [ ] 运行 Clippy 静态分析
- [ ] 运行代码格式化检查
- [ ] 运行依赖安全扫描
- [ ] 更新 `Cargo.toml` 中的版本号
- [ ] 编译所有平台的二进制文件
- [ ] 创建所有平台的压缩包
- [ ] 验证所有压缩包
- [ ] 创建并推送 Git 标签
- [ ] 在 GitHub 上创建 Release
- [ ] 上传所有压缩包
- [ ] 下载并测试一个二进制文件
- [ ] 测试功能完整性

## 7. 版本号规则

遵循语义化版本规范：
- **MAJOR**：不兼容的 API 变更
- **MINOR**：向下兼容的新功能
- **PATCH**：向下兼容的 bug 修复

例如：
- 从 v0.1.0 到 v0.2.0：添加了新功能
- 从 v0.2.0 到 v0.2.1：修复了 bug
- 从 v0.2.1 到 v1.0.0：进行了不兼容的 API 变更

---

本文档将帮助您顺利完成 EpubSmith 项目预编译二进制文件的发布工作。如有任何问题，请在 GitHub 上提交 Issue。