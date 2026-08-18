# 解析规则

## 范围

定义 EpubSmith 如何把纯文本解析为章节结构：规则文件（TOML）的字段语义、内置默认规则、输入编码处理与 `doctor` 诊断。解决「同一段文本在不同章节格式下如何被正确分章」的问题。

不涉及输出样式（见 [styling](../styling/)）与命令参数用法（见 [cli](../cli/)）。

## 行为契约

### 规则文件（TOML）

通过 `-r <文件>` 指定。结构对应 `src/models.rs` 的 `Rules`：

```toml
# 章节检测正则（满足任一即视为章节起始行）
[chapter]
regex = [
    "^[\t\\s]*[0-9]+章",
    "^[\t\\s]*第[零〇一二三四五六七八九十百千万两0-9]+章"
]

# 从章节标题提取序号（按数组顺序优先匹配）
[chapter_number_extraction]
rules = [
    { pattern = "^[\t\\s]*([0-9]+)章", capture_group = 1, number_type = "arabic" },
    { pattern = "^[\t\\s]*第([零〇一二三四五六七八九十百千万两0-9]+)章", capture_group = 1, number_type = "auto" }
]

# 段落处理
[paragraph]
merge_lines = false      # 是否合并连续非空行为一段
trim_whitespace = true   # 是否修剪行首行尾空白

# 元数据（可选，命令行 --title/--author 会覆盖）
[meta]
title = "书名"
author = "作者"
language = "zh-CN"
```

| 段 | 字段 | 说明 |
|----|------|------|
| `[chapter]` | `regex` | 分章正则数组，任一匹配即开始新章 |
| `[chapter_number_extraction]` | `rules` | 序号提取规则：`pattern` 正则、`capture_group` 捕获组（从 1 开始）、`number_type`（`auto`/`arabic`/`chinese`/`roman`/`english`） |
| `[paragraph]` | `merge_lines` | `true` 用空格合并连续非空行，`false` 保留换行 |
| `[paragraph]` | `trim_whitespace` | 是否去除行首行尾空白 |
| `[meta]` | `title`/`author`/`language` | 书籍元数据，命令行参数优先 |

### 内置默认规则

不指定 `-r` 时生效（v0.2.0+）：

- 默认章节单位：`章 / 节 / 卷 / 回`；默认**不**将 `张` 视为章节单位，降低正文误判风险
- 默认支持完整中文数字范围：`零〇一二三四五六七八九十百千万两0-9`
- 若文本确实使用 `第1张` 格式，需通过 `--rules` 自定义规则显式启用

### 输入编码处理

1. 指定 `--encoding` 时直接使用指定编码解码
2. 未指定时读取文件前 1024 字节，用 chardet 探测编码
3. 置信度 > 80% 使用探测结果，否则回退 UTF-8

### 章节解析

1. 编译 `[chapter]` 正则
2. 遍历文件行，命中正则即记录新章节起始位置
3. 未找到任何章节时将整个文件作为一个章节

### doctor 诊断

`doctor <INPUT>` 输出：
- 章节数据异常统计与样本（重复章节号、倒序章节号）
- 规则风险提示（「张」单位误判风险、中文数字范围不足风险）
- 建议执行的 `preview` / `convert` 命令

## 失败与边界

- **规则文件非法**：TOML 解析失败或正则编译失败时在加载阶段报错，转换不继续。
- **无章节文件**：整文件作为单章节处理，不报错。
- **章节号错乱**：源文本章节号不递增时，仅检测并告警，不阻断转换。
- **编码探测失败**：置信度不足时按 UTF-8 解码，可能出现乱码；用 `--encoding` 显式指定可消除。

## 相关链接

- [cli](../cli/) — `-r` / `--encoding` / `doctor` 参数用法
- [architecture](../../architecture.md) — 解析在数据流中的位置
- [styling](../styling/) — 解析结果如何被渲染为 XHTML
