# 打包、校验与发布

## 范围

定义解析+渲染之后的收尾链路：EPUB 打包（ZIP 结构、manifest/spine/NCX）、`--check` 校验、构建发布流程与测试策略。解决「XHTML + CSS 如何变成合法可分发的 .epub 文件」的问题。

不涉及解析规则（见 [rules](../rules/)）与样式封面（见 [styling](../styling/)）。

## 行为契约

### 渲染产物

| 产物 | 说明 |
|------|------|
| `chapter_NNN.xhtml` | Tera 模板渲染章节，`.container` 包裹标题与段落 |
| `cover.xhtml` | 封面页（`--cover auto` 或用户图片时生成，spine 首位） |
| `nav.xhtml` | EPUB3 目录 |
| `toc.ncx` | EPUB2 兼容目录（navMap 一层结构，章节链接补零） |
| `style.css` | 默认或 `--style` 自定义样式 |
| `content.opf` | manifest / spine / guide，注册封面与 NCX |
| `mimetype` + `META-INF/container.xml` | EPUB 容器 |

### 打包流程

1. `mimetype`（`application/epub+zip`）为首个 ZIP 条目且 **Stored 无压缩**（EPUB 规范要求）
2. 其余文件 Deflate 压缩
3. 打包为 `.epub`
4. `--check` 时调用 epubcheck 校验

### EPUB 校验（--check）

- 需要 Java 运行时
- 优先使用当前目录的 `epubcheck.jar`，其次 PATH 上的 `epubcheck` 命令
- 均不可用时给出安装指引（https://github.com/w3c/epubcheck）

### 构建

```bash
cargo build           # debug
cargo build --release # 发布
```

发布构建脚本：`scripts/build-release.sh`（交叉编译各平台二进制并打压缩包）。

### 发布流程

1. 更新 `Cargo.toml` 版本号（语义化版本 MAJOR.MINOR.PATCH）
2. 更新变更日志（迁移后记入 `.agents/notes/`，见根 [AGENTS.md](../../../AGENTS.md)）
3. 运行质量检查与依赖安全扫描
4. 运行 `scripts/build-release.sh` 生成各平台压缩包
5. 创建并推送 Git 标签（`git tag -a vX.Y.Z -m "..."`）
6. 在 GitHub Releases 上传压缩包并填写发布说明
7. 下载验证：`./epub-smith --version` + 试转换

### 分发

- **cargo install**：`cargo install epub-smith`
- **预编译二进制**：GitHub Release 提供 macOS（arm64/amd64）、Linux（amd64）、Windows（amd64）压缩包，单一可执行文件

### 测试策略

- 单元测试：编码处理、章节解析、XHTML 渲染、封面生成
- 集成测试：完整转换流程（含封面、NCX、导航链接）
- 边界测试：空文件、无章节文件、非法规则文件

## 失败与边界

- **mimetype 顺序违规**：若 `mimetype` 不是首个 ZIP 条目或非 Stored，生成的 EPUB 不符合规范，阅读器可能拒绝打开。
- **epubcheck 不可用**：`--check` 给出安装指引，转换本身不失败。
- **无 Java 环境**：epubcheck 无法运行，校验被跳过。
- **跨平台差异**：Windows / macOS / Linux 均支持；发布二进制按平台分别构建。

## 相关链接

- [cli](../cli/) — `--check` 等参数用法
- [styling](../styling/) — 渲染产物与样式
- [architecture](../../architecture.md) — 打包在数据流中的位置
