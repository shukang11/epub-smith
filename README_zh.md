# EpubSmith

一个用 Rust 编写的可预测、可解释、可复用的 TXT 转 EPUB 命令行工具。

**核心哲学**：专注于做好一件事 - 生成标准化、稳定的 EPUB 文件。对于其他格式，使用下游专业工具配合我们推荐的工作流。

[![English Version](https://img.shields.io/badge/README-English%20Version-blue)](README.md)

## 目录

- [特性](#特性)
- [安装](#安装)
- [快速开始](#快速开始)
- [命令结构](#命令结构)
- [自定义样式](#自定义样式)
- [调试和性能](#调试和性能)
- [测试](#测试)
- [推荐工作流](#推荐工作流)
- [贡献](#贡献)
- [许可证](#许可证)
- [致谢](#致谢)
- [联系方式](#联系方式)

## 特性

- **可预测的结果**：将纯文本文件转换为结构良好的 EPUB 书籍，输出一致
- **易于使用**：简单的命令行界面，带有合理的默认值
- **可定制**：支持自定义解析规则和模板
- **章节连贯性检查**：自动验证章节编号的一致性
- **性能优化**：快速解析和生成，提供性能指标
- **调试工具**：空运行模式、章节大纲预览和详细的性能分析
- **自定义样式**：支持自定义 CSS 样式
- **EPUB 验证**：可选集成 epubcheck 进行验证
- **多输入文件**：将多个 TXT 文件合并为单个 EPUB 书籍
- **STDIN 支持**：从标准输入读取内容，支持管道工作流
- **结构快照**：导出和导入章节结构，以便重用和手动调整
- **输入验证**：使用预览命令诊断文本结构问题

## 安装

### 使用安装脚本

安装 EpubSmith 最简单的方法是使用提供的安装脚本。该脚本会自动：
- 从 GitHub 检测最新发布版本
- 识别您的操作系统和架构
- 下载合适的二进制包
- 安装到合适的位置
- 如有需要，配置您的 PATH 环境变量

#### 快速安装

```bash
curl -fsSL https://raw.githubusercontent.com/shukang11/epub-smith/main/install.sh | bash
```

这将在您的系统上安装最新版本的 EpubSmith。

#### 安装特定版本

```bash
curl -fsSL https://raw.githubusercontent.com/shukang11/epub-smith/main/install.sh | bash -s -- --version v0.2.0
```

#### 仅下载不安装

如果您只想下载二进制文件而不安装：

```bash
curl -fsSL https://raw.githubusercontent.com/shukang11/epub-smith/main/install.sh | bash -s -- --download-only
```

#### 检查安装

安装完成后，您可以通过运行以下命令验证 EpubSmith 是否已正确安装：

```bash
epub-smith --version
# 或使用短名称
epbs --version
```

您应该会看到已安装的版本号。要获取帮助，请运行：

```bash
epub-smith --help
# 或
epbs --help
```

### 从源代码安装

1. 确保您已安装 Rust 和 Cargo。您可以通过 [rustup](https://rustup.rs/) 安装它们。
2. 克隆仓库：
   ```bash
   git clone https://github.com/shukang11/epub-smith.git
   cd epub-smith
   ```
3. 构建项目：
   ```bash
   cargo build --release
   ```
4. 可执行文件将在 `target/release/epub-smith` 位置可用



## 快速开始

### 基本用法

将文本文件转换为 EPUB：

```bash
epub-smith convert my_book.txt
```

或使用短名称：

```bash
epbs convert my_book.txt
```

这两个命令都会在当前目录生成 `my_book.epub`。

### 自定义输出文件

```bash
epub-smith convert my_book.txt -o my_custom_book.epub
```

### 指定作者和标题

```bash
epub-smith convert my_book.txt --author "张三" --title "我的书"
```

### 预览章节（空运行）

预览 EpubSmith 如何解析您的文件，而不生成 EPUB：

```bash
epub-smith preview dry-run my_book.txt
```

或使用语言设置：

```bash
epub-smith --lang zh-CN preview dry-run my_book.txt
```

### 打印章节大纲

```bash
epub-smith preview outline my_book.txt
```

### 多个输入文件

将多个 TXT 文件合并为单个 EPUB 书籍：

```bash
# 合并 src 目录中的所有 TXT 文件
epub-smith convert src/*.txt -o combined_book.epub

# 按顺序合并特定文件
epub-smith convert chapter1.txt chapter2.txt chapter3.txt -o book.epub
```

### STDIN 支持

从标准输入读取内容，支持管道工作流：

```bash
# 直接将内容管道传输到 EpubSmith
cat novel.txt | epub-smith convert - -o novel.epub

# 与其他 CLI 工具一起使用
grep -v "#" raw.txt | sed 's/\r//g' | epub-smith convert - -o cleaned.epub
```

### 结构快照

将章节结构导出到 JSON 文件，以便重用和手动调整。快照包含完整的书籍信息，包括元数据和章节内容。

#### 生成快照

```bash
# 从 TXT 文件导出结构快照
epub-smith snapshot save novel.txt -o structure.json
```

#### 使用快照

```bash
# 使用现有结构快照生成 EPUB（不需要 TXT 文件）
epub-smith snapshot load structure.json -o custom_book.epub

# 手动编辑 structure.json 调整章节，然后重新使用
```

### 自定义规则

使用自定义规则文件控制 EpubSmith 如何解析您的文本：

```bash
epub-smith convert my_book.txt -r custom_rules.toml
```

 anthology 格式（`01【标题】`）的示例规则文件：

```toml
[chapter]
regex = [
    "^[\t\s]*[0-9]+【",
    "^[\t\s]*[０-９]+【",
    "^[\t\s]*[ⅠⅡⅢⅣⅤⅥⅦⅧⅨⅩ]+【"
]

[chapter_number_extraction]
rules = [
    { pattern = "^[\t\s]*([0-9]+)【", capture_group = 1, number_type = "arabic" }
]

[paragraph]
merge_lines = false
trim_whitespace = true
```

## 命令结构

EpubSmith 使用基于子命令的 CLI 结构：

```
epub-smith [全局选项] <子命令> [子命令选项]
```

### 全局选项

| 选项 | 描述 |
|------|------|
| `-v, --verbose` | 显示详细日志 |
| `--debug` | 启用性能分析 |
| `--lang <LANGUAGE>` | 指定输出语言（例如，en, zh-CN） |

### 子命令

| 子命令 | 描述 |
|--------|------|
| `convert` | 将 TXT 文件转换为 EPUB（默认命令） |
| `template export <DIRECTORY>` | 将样式模板导出到目录 |
| `snapshot save <INPUT> -o <FILE>` | 将章节结构保存到快照文件 |
| `snapshot load <FILE> -o <OUTPUT>` | 从快照文件生成 EPUB |
| `preview outline <INPUT>` | 打印章节大纲 |
| `preview dry-run <INPUT>` | 显示详细的章节结构，不生成 EPUB |

### 转换选项

| 选项 | 描述 |
|------|------|
| `<INPUT>` | 输入 TXT 文件或目录（使用 `-` 表示 STDIN） |
| `-r, --rules <FILE>` | 规则文件路径 |
| `-o, --output <FILE>` | 输出 EPUB 文件路径 [默认值: book.epub] |
| `-e, --encoding <ENCODING>` | 强制输入文件编码 |
| `--title <TITLE>` | 指定书籍标题 |
| `--author <AUTHOR>` | 指定书籍作者 |
| `--cover <FILE>` | 指定封面图片路径 |
| `--language <LANGUAGE>` | 指定书籍语言 [默认值: zh-CN] |
| `--check` | 使用 epubcheck 验证 EPUB |
| `--style <FILE>` | 指定自定义 CSS 样式文件 |

## 自定义样式

EpubSmith 使用 Tera 模板和 CSS 进行样式设计。您可以导出默认模板并修改它们：

```bash
epub-smith template export my_templates
```

这将创建一个包含默认模板和 CSS 文件的目录。然后您可以修改这些文件，并将它们与您的自定义 CSS 一起使用：

```bash
epub-smith convert input.txt --style my_templates/default.css
```



## 调试和性能

### 空运行

`preview-dry-run` 子命令允许您预览 EpubSmith 将如何解析您的文件，而不生成 EPUB：

```bash
epub-smith preview dry-run input.txt
```

### 详细日志

使用 `--verbose` 查看转换过程中的详细日志：

```bash
epub-smith --verbose convert input.txt
```

或使用短名称：

```bash
epbs --verbose convert input.txt
```

### 性能分析

`--debug` 选项启用性能指标，显示转换每个阶段所花费的时间：

```bash
epub-smith --debug convert input.txt
```

## 测试

运行测试套件以确保一切正常工作：

```bash
cargo test
```

## 推荐工作流

EpubSmith 专注于生成标准化、稳定的 EPUB 文件。对于其他格式，我们建议使用专业的下游工具。以下是一些常见的工作流：

### EPUB → Kindle (MOBI/KFX)

```bash
# 步骤 1：使用 EpubSmith 生成 EPUB
epub-smith convert novel.txt -o novel.epub

# 步骤 2：使用 Calibre 的 ebook-convert 转换为 MOBI
ebook-convert novel.epub novel.mobi

# 步骤 3：通过电子邮件或 Calibre 发送到 Kindle
```

### EPUB → PDF

```bash
# 步骤 1：使用 EpubSmith 生成 EPUB
epub-smith convert novel.txt -o novel.epub

# 步骤 2：使用 Calibre 的 ebook-convert 转换为 PDF
ebook-convert novel.epub novel.pdf

# 或使用 pandoc 进行更多自定义
pandoc novel.epub -o novel.pdf --pdf-engine=xelatex
```

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证 - 有关详细信息，请参阅 [LICENSE](LICENSE) 文件。

## 致谢

- 本项目使用 [Tera](https://tera.netlify.app/) 模板引擎
- EPUB 生成由 [zip](https://crates.io/crates/zip) crate 提供支持
- 命令行界面使用 [clap](https://clap.rs/) 构建

## 联系方式

如有问题、建议或问题，请在 GitHub 上打开一个 [issue](https://github.com/shukang11/epub-smith/issues)。
