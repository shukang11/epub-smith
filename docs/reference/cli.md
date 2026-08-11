# CLI 参数参考

## 命令行结构

EpubSmith 使用基于子命令的命令行结构：

```
epub-smith [全局选项] <子命令> [子命令选项]
```

## 全局选项

| 选项 | 描述 |
|------|------|
| `-v, --verbose` | 显示详细日志 |
| `--debug` | 启用性能分析，显示各阶段耗时 |
| `--lang <语言>` | 指定输出语言（如：en, zh-CN） |

## 子命令

### doctor - 诊断解析风险（推荐先执行）

在转换前分析输入文本的章节质量，输出问题分类、证据样本与建议命令。

```bash
epub-smith doctor <INPUT> [OPTIONS]
```

**参数：**

| 参数 | 描述 |
|------|------|
| `<INPUT>` | 输入 TXT 文件或目录，使用 `-` 表示从标准输入读取 |

**选项：**

| 选项 | 简写 | 类型 | 描述 | 默认值 |
|------|------|------|------|--------|
| `--rules` | `-r` | 文件路径 | 规则文件路径 | - |
| `--encoding` | `-e` | 字符串 | 强制输入文件编码 | 自动探测 |
| `--max-samples` | - | 数字 | 每类问题输出的样本上限 | `5` |

**示例：**

```bash
# 使用默认规则做诊断
epub-smith doctor my_book.txt

# 使用自定义规则做诊断
epub-smith doctor my_book.txt -r rules.toml

# 指定编码并增加样本
epub-smith doctor my_book.txt -e gbk --max-samples 10
```

### convert - 转换为 EPUB（默认命令）

TXT 文件转换为 EPUB 的主要命令。

```bash
epub-smith convert <INPUT> [OPTIONS]
```

**参数：**

| 参数 | 描述 |
|------|------|
| `<INPUT>` | 输入 TXT 文件或目录，使用 `-` 表示从标准输入读取 |

**选项：**

| 选项 | 简写 | 类型 | 描述 | 默认值 |
|------|------|------|------|--------|
| `--rules` | `-r` | 文件路径 | 规则文件路径 | - |
| `--output` | `-o` | 文件路径 | 输出 EPUB 文件路径 | `book.epub` |
| `--encoding` | `-e` | 字符串 | 强制输入文件编码 | 自动探测 |
| `--title` | - | 字符串 | 指定书名 | 从文件名获取 |
| `--author` | - | 字符串 | 指定作者 | `Unknown` |
| `--cover` | - | 路径或 `auto` | 指定封面图片路径，或 `auto` 自动生成默认封面 | - |
| `--language` | - | 字符串 | 指定书籍语言 | `zh-CN` |
| `--check` | - | 标志 | 调用 epubcheck 校验 | `false` |
| `--style` | - | 文件路径 | 指定自定义 CSS 样式文件 | - |

**示例：**

```bash
# 基本转换
epub-smith convert my_book.txt

# 指定输出文件名
epub-smith convert my_book.txt -o output.epub

# 批量合并文件
epub-smith convert chapter1.txt chapter2.txt chapter3.txt -o book.epub

# 使用自定义规则
epub-smith convert my_book.txt -r rules.toml

# 从 STDIN 读取
cat book.txt | epub-smith convert - -o book.epub

# 自动生成默认封面（离线 SVG 渲染，无网络/模型依赖）
epub-smith convert my_book.txt --cover auto
```

### template export - 导出模板

导出默认模板到指定目录，用于自定义样式。

```bash
epub-smith template-export <DIRECTORY>
```

**示例：**

```bash
epub-smith template-export my_templates
```

这将导出以下文件：
- `style.css` - 样式文件，可编辑修改
- `preview.html` - 预览文件，可在浏览器中打开（含 6 个示例章节 + 目录/翻页交互 + 示例封面，编辑 style.css 后刷新即可见新效果）
- `cover.svg` - 封面模板（`--cover auto` 使用），可编辑配色与装饰
- `README.md` - 使用说明

### snapshot save - 保存快照

将章节结构保存到 JSON 文件，可用于后续生成 EPUB 或手动调整。

```bash
epub-smith snapshot-save <INPUT> [OPTIONS]
```

**参数：**

| 参数 | 描述 |
|------|------|
| `<INPUT>` | 输入 TXT 文件或目录 |

**选项：**

| 选项 | 简写 | 类型 | 描述 |
|------|------|------|------|
| `--rules` | `-r` | 文件路径 | 规则文件路径 |
| `--output` | `-o` | 文件路径 | **必需** - 快照输出文件路径 |

**示例：**

```bash
# 保存章节结构
epub-smith snapshot-save my_book.txt -o structure.json

# 使用自定义规则
epub-smith snapshot-save my_book.txt -r rules.toml -o structure.json
```

### snapshot load - 从快照生成

使用已保存的快照生成 EPUB，无需原始 TXT 文件。

```bash
epub-smith snapshot-load <FILE> [OPTIONS]
```

**参数：**

| 参数 | 描述 |
|------|------|
| `<FILE>` | 快照 JSON 文件路径 |

**选项：**

| 选项 | 简写 | 类型 | 描述 | 默认值 |
|------|------|------|------|--------|
| `--output` | `-o` | 文件路径 | 输出 EPUB 文件路径 | `book.epub` |
| `--rules` | `-r` | 文件路径 | 规则文件路径 | - |
| `--author` | - | 字符串 | 指定作者 | 快照中的值 |
| `--title` | - | 字符串 | 指定书名 | 快照中的值 |
| `--language` | - | 字符串 | 指定语言 | `zh-CN` |
| `--check` | - | 标志 | 调用 epubcheck 校验 | `false` |
| `--style` | - | 文件路径 | 指定 CSS 样式文件 | - |
| `--cover` | - | 路径或 `auto` | 封面图片路径，或 `auto` 自动生成默认封面 | - |

**示例：**

```bash
# 从快照生成 EPUB
epub-smith snapshot-load structure.json

# 指定输出文件名
epub-smith snapshot-load structure.json -o custom_book.epub

# 手动编辑 snapshot 后重新生成
epub-smith snapshot-load structure.json -o revised.epub
```

### preview outline - 预览章节大纲

打印检测到的章节结构概览，不生成 EPUB。

```bash
epub-smith preview-outline <INPUT> [OPTIONS]
```

**参数：**

| 参数 | 描述 |
|------|------|
| `<INPUT>` | 输入 TXT 文件或目录 |

**选项：**

| 选项 | 简写 | 类型 | 描述 |
|------|------|------|------|
| `--rules` | `-r` | 文件路径 | 规则文件路径 |

**示例：**

```bash
# 查看章节大纲
epub-smith preview-outline my_book.txt

# 使用自定义规则
epub-smith preview-outline my_book.txt -r rules.toml
```

### preview dry-run - 预览章节详情

显示详细的章节结构检测结果，包括章节号、行号等信息。

```bash
epub-smith preview-dry-run <INPUT> [OPTIONS]
```

**参数：**

| 参数 | 描述 |
|------|------|
| `<INPUT>` | 输入 TXT 文件或目录 |

**选项：**

| 选项 | 简写 | 类型 | 描述 |
|------|------|------|------|
| `--rules` | `-r` | 文件路径 | 规则文件路径 |

**示例：**

```bash
# 预览章节详情
epub-smith preview-dry-run my_book.txt

# 使用自定义规则
epub-smith preview-dry-run my_book.txt -r rules.toml

# 与全局选项结合
epub-smith --lang zh-CN preview-dry-run my_book.txt
```

**输出示例：**

```
✅ 章节号是递增的，符合要求
检测到的章节：

[001] 第1章 开篇 (lines 1-50)
[002] 第2章 发展 (lines 51-100)
[003] 第3章 高潮 (lines 101-150)
```

## 完整使用示例

### 短篇小说合集处理

```bash
# 1. 先预览检查章节识别是否正确
epub-smith --lang zh-CN preview-dry-run anthology.txt -r rules_anthology.toml

# 2. 确认无误后生成 EPUB
epub-smith --lang zh-CN convert anthology.txt -r rules_anthology.toml -o anthology.epub
```

### 使用快照工作流

```bash
# 1. 保存当前解析结果
epub-smith snapshot-save draft.txt -o draft_snapshot.json

# 2. 手动编辑 snapshot（调整章节顺序、修改标题等）
# 使用文本编辑器修改 draft_snapshot.json

# 3. 从修改后的 snapshot 生成 EPUB
epub-smith snapshot-load draft_snapshot.json -o revised_book.epub
```

### 自定义规则文件示例

处理 `01【标题】` 格式的短篇小说合集：

```toml
# anthology_rules.toml

[chapter]
regex = [
    "^[\t\\s]*[0-9]+【",
    "^[\t\\s]*[０-９]+【",
    "^[\t\\s]*[ⅠⅡⅢⅣⅤⅥⅦⅧⅨⅩ]+【"
]

[chapter_number_extraction]
rules = [
    { pattern = "^[\t\\s]*([0-9]+)【", capture_group = 1, number_type = "arabic" },
    { pattern = "^[\t\\s]*([０-９]+)【", capture_group = 1, number_type = "arabic" },
    { pattern = "^[\t\\s]*([ⅠⅡⅢⅣⅤⅥⅦⅧⅨⅩ]+)【", capture_group = 1, number_type = "roman" }
]

[paragraph]
merge_lines = false
trim_whitespace = true
```

### 默认规则说明（v0.2.0+）

- 默认章节单位为：`章 / 节 / 卷 / 回`
- 默认**不**将 `张` 视为章节单位，以降低正文误判风险
- 默认支持更完整的中文数字范围：`零〇一二三四五六七八九十百千万两0-9`
- 若你的文本确实使用 `第1张` 作为章节格式，请通过 `--rules` 自定义规则显式启用

## 注意事项

1. **全局选项位置**：全局选项（`--verbose`, `--debug`, `--lang`）必须放在子命令之前
2. **必需参数**：`snapshot-save` 需要使用 `-o` 指定输出文件
3. **规则文件**：复杂文本或特殊章节格式（如 `第1张`）建议使用自定义规则文件
4. **编码问题**：如果文件编码不是 UTF-8，请使用 `--encoding` 选项指定
