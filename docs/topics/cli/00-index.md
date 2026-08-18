# CLI 契约

## 范围

定义 `epub-smith` 命令行接口：全局选项、子命令、参数默认值与典型用法。解决「用户如何调用工具完成一次 TXT→EPUB 转换或诊断」的问题。

不涉及解析规则文件的字段语义（见 [rules](../rules/)）、样式与封面（见 [styling](../styling/)）、打包与发布（见 [delivery](../delivery/)）。

## 行为契约

### 全局选项（置于子命令之前）

| 选项 | 描述 |
|------|------|
| `-v, --verbose` | 显示详细日志 |
| `--debug` | 启用性能分析，显示各阶段耗时 |
| `--lang <语言>` | 指定输出语言（如 en, zh-CN） |

### 子命令一览

| 子命令 | 作用 |
|--------|------|
| `doctor` | 转换前诊断输入文本的章节质量，输出问题分类、证据样本与建议命令 |
| `convert` | TXT → EPUB（默认命令） |
| `template export` | 导出默认模板（style.css / preview.html / cover.svg / README.md）到目录 |
| `snapshot save` | 将章节结构保存为 JSON 快照（含元数据与章节内容） |
| `snapshot load` | 从快照生成 EPUB，无需原始 TXT |
| `preview outline` | 打印检测到的章节大纲 |
| `preview dry-run` | 显示详细章节结构（章节号、行号等），不生成 EPUB |

### convert 选项

| 选项 | 简写 | 类型 | 描述 | 默认值 |
|------|------|------|------|--------|
| `<INPUT>` | - | 路径 | 输入 TXT 文件或目录，多个输入按序合并；`-` 表示 STDIN | - |
| `--rules` | `-r` | 文件路径 | 规则文件路径 | 内置默认规则 |
| `--output` | `-o` | 文件路径 | 输出 EPUB 路径 | `book.epub` |
| `--encoding` | `-e` | 字符串 | 强制输入编码 | 自动探测 |
| `--title` | - | 字符串 | 指定书名 | 从文件名获取 |
| `--author` | - | 字符串 | 指定作者 | `Unknown` |
| `--cover` | - | 路径或 `auto` | 封面图片，或 `auto` 自动生成默认封面 | 无封面 |
| `--language` | - | 字符串 | 书籍语言 | `zh-CN` |
| `--check` | - | 标志 | 调用 epubcheck 校验 | `false` |
| `--style` | - | 文件路径 | 自定义 CSS 样式文件 | 默认样式 |

### doctor 选项

| 选项 | 简写 | 类型 | 描述 | 默认值 |
|------|------|------|------|--------|
| `<INPUT>` | - | 路径 | 输入 TXT 文件或目录；`-` 表示 STDIN | - |
| `--rules` | `-r` | 文件路径 | 规则文件路径 | - |
| `--encoding` | `-e` | 字符串 | 强制输入编码 | 自动探测 |
| `--max-samples` | - | 数字 | 每类问题输出样本上限 | `5` |

### snapshot / preview 选项

- `snapshot save <INPUT> -o <FILE>`：`-o` **必需**；`-r` 可选
- `snapshot load <FILE>`：`-o`（默认 `book.epub`）、`-r`、`--title`、`--author`（默认取快照值）、`--language`（默认 `zh-CN`）、`--check`、`--style`、`--cover`
- `preview outline <INPUT>` / `preview dry-run <INPUT>`：`-r` 可选

### 典型工作流

```bash
# 先诊断再转换（复杂文本建议顺序执行）
epub-smith doctor my_book.txt -r rules.toml
epub-smith convert my_book.txt -r rules.toml -o book.epub

# 快照工作流：保存 → 手动调整 JSON → 重新生成
epub-smith snapshot save draft.txt -o draft.json
epub-smith snapshot load draft.json -o revised.epub

# 多文件合并 + 从 STDIN 读取
epub-smith convert ch1.txt ch2.txt ch3.txt -o book.epub
cat novel.txt | epub-smith convert - -o novel.epub

# 离线默认封面
epub-smith convert my_book.txt --cover auto
```

## 失败与边界

- **全局选项位置**：`--verbose` / `--debug` / `--lang` 必须放在子命令之前，否则不生效。
- **必需参数**：`snapshot save` 缺 `-o` 报错退出。
- **规则文件**：缺失或非法（正则编译失败）时报错并提示；复杂文本或特殊章节格式（如 `第1张`）建议自定义规则文件。
- **编码**：非 UTF-8 输入需 `--encoding` 指定，否则按自动探测结果解码。
- **`--check` 依赖 Java**：优先用当前目录 `epubcheck.jar`，其次 PATH 上的 `epubcheck`，均不可用时给出安装指引。
- **`--cover auto` 依赖系统中文字体**：按 font-family 匹配 PingFang / Microsoft YaHei / Noto Sans CJK SC 等，跨设备渲染可能不同。

## 相关链接

- [rules](../rules/) — 规则文件字段语义与默认规则
- [styling](../styling/) — `--style` / `--cover` 的行为细节
- [delivery](../delivery/) — `--check` 校验机制与发布流程
- [architecture](../../architecture.md) — 命令到解析/渲染/打包的主链路
