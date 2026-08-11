# 构建与发布

## 构建

本地开发构建：

```bash
cargo build           # debug
cargo build --release # 发布
```

发布构建脚本：`scripts/build-release.sh`（交叉编译各平台二进制并打压缩包）。

发布前质量检查：

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
cargo audit   # 依赖安全扫描（需安装 cargo-audit）
```

## 发布流程

1. 更新 `Cargo.toml` 版本号（语义化版本 MAJOR.MINOR.PATCH）
2. 更新 `docs/llm_ctx/CHANGELOG.md`
3. 运行质量检查与依赖安全扫描
4. 运行 `scripts/build-release.sh` 生成各平台压缩包
5. 创建并推送 Git 标签（`git tag -a vX.Y.Z -m "..."`）
6. 在 GitHub Releases 上传压缩包并填写发布说明
7. 下载验证：`./epub-smith --version` + 试转换

### 发布清单

- [ ] 测试 / clippy / fmt 通过
- [ ] 版本号更新
- [ ] CHANGELOG 更新
- [ ] 依赖安全扫描（cargo audit）
- [ ] 各平台二进制生成
- [ ] Git 标签 + GitHub Release
- [ ] 下载验证

## 分发

- **cargo install**：`cargo install epub-smith`
- **预编译二进制**：GitHub Release 提供 macOS（arm64/amd64）、Linux（amd64）、Windows（amd64）压缩包，单一可执行文件
- **平台支持**：Windows / macOS / Linux

## EPUB 校验

`convert --check` 调用 epubcheck 校验生成的 EPUB：
- 需要 Java 运行时；优先使用当前目录的 `epubcheck.jar`，其次 PATH 上的 `epubcheck` 命令
- 均不可用时给出安装指引（https://github.com/w3c/epubcheck）

## 测试策略

- 单元测试：编码处理、章节解析、XHTML 渲染、封面生成
- 集成测试：完整转换流程（含封面、NCX、导航链接）
- 边界测试：空文件、无章节文件、非法规则文件
