# 配置项参考

EpubSmith 的配置分两部分：**规则文件**（TOML，控制解析）和**命令行参数**（控制输出/样式/封面）。

## 规则文件（TOML）

通过 `-r <文件>` 指定。结构（对应 `models.rs` 的 `Rules`）：

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

### 字段说明

| 段 | 字段 | 说明 |
|----|------|------|
| `[chapter]` | `regex` | 分章正则数组，任一匹配即开始新章 |
| `[chapter_number_extraction]` | `rules` | 序号提取规则：`pattern` 正则、`capture_group` 捕获组（从 1 开始）、`number_type`（`auto`/`arabic`/`chinese`/`roman`/`english`） |
| `[paragraph]` | `merge_lines` | `true` 用空格合并连续非空行，`false` 保留换行 |
| `[paragraph]` | `trim_whitespace` | 是否去除行首行尾空白 |
| `[meta]` | `title`/`author`/`language` | 书籍元数据，命令行参数优先 |

不指定 `-r` 时使用内置默认规则（章/节/卷/回 + 番外/序/尾声/后记/附录/正文，禁用"张"单位）。

## 命令行参数（非规则文件）

样式与封面通过命令行控制，不写入规则文件：

| 参数 | 说明 | 参考 |
|------|------|------|
| `--style <文件>` | 自定义 CSS 样式文件 | 见 `templates.md` |
| `--cover auto` | 自动生成默认封面 | 见 `templates.md` |
| `--cover <文件>` | 指定封面图片 | 见 `templates.md` |
| `--title` / `--author` / `--language` | 覆盖书籍元数据 | 见 `cli.md` |

完整命令参数见 `cli.md`。
