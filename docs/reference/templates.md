# 模板与样式参考

（在此维护模板结构、变量与样式说明。）

## 封面模板（cover.svg）

`template export` 导出 `cover.svg`，供 `--cover auto` 自动生成默认封面时使用。

**占位符：**

| 占位符 | 说明 |
|--------|------|
| `{{ title_lines }}` | 书名（自动切行，最多 3 行，每行 ≤14 字符） |
| `{{ author }}` | 作者 |

**可自定义项：** 背景渐变（`linearGradient`）、装饰线/点、字体、配色。

**说明：**
- 编辑本文件后，配合 `--cover auto` 生效（当前版本封面模板为内嵌默认，自定义 cover.svg 作为生成源的接线待后续版本开放）。
- SVG 为 XML，注释内不允许出现 `--` 序列。
- 渲染依赖系统中文字体（PingFang SC / Microsoft YaHei / Noto Sans CJK SC 等，按 font-family fallback 匹配）。
