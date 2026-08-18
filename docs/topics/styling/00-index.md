# 模板与样式

## 范围

定义 EPUB 的视觉呈现：默认样式方案、`style.css` 自定义、`preview.html` 预览页、封面（`--cover auto` / 用户图片）与 `cover.svg` 模板。解决「生成的书长什么样、如何调整」的问题。

不涉及规则文件与解析（见 [rules](../rules/)）、打包与校验（见 [delivery](../delivery/)）。

## 行为契约

### 默认样式方案（方案 C，可读性优先）

- 正文：首行缩进 `2em`、段后距 `0.95em`、两端对齐、`white-space: pre-line` 保留段内换行
- 正文基准：衬线字体族、18px、行高 1.8
- 章节标题：居中、深色（默认 CSS 为 1.45em + 全宽灰线）

### 样式自定义

`template export` 导出 `style.css`，编辑后用 `--style <文件>` 生效：

```bash
epub-smith convert novel.txt --style my_tpl/style.css -o out.epub
```

| 选择器 | 控制内容 |
|--------|----------|
| `body` | 全局字体族、字号、行高、文字/背景色 |
| `.container` | 章节/目录页内容容器：max-width、居中、内边距 |
| `h1.chapter-title` | 章节标题：字号、字重、对齐、字距、颜色 |
| `p.chapter-content` | 正文段落：首行缩进、段距、对齐、段内换行 |
| `.cover-page` | 封面页整体：居中、背景色 |
| `.cover-image` | 封面图：自适应宽度、不拉伸变形 |
| `nav#toc` | 目录导航：标题、列表、链接样式 |
| `@media print` | 打印/分页行为（`page-break-before: always` 每章分页） |

### 页面结构

```
章节页：<body> > div.container > (h1.chapter-title + p.chapter-content)
目录页：<body> > div.container > nav#toc
封面页：<body.cover-page> > img.cover-image
```

### 预览页（preview.html）

`template export` 导出，用于在浏览器预览 `style.css` 效果：

- 顶部目录（固定）+ 翻页控件（◀/▶、键盘 ←→）+ 示例封面 + 6 个示例章节（小说/散文/科技/超长标题/对话/短段落）
- 内嵌 `style.css`，编辑后重新注入并刷新即可对照效果

### 封面

| 方式 | 命令 | 说明 |
|------|------|------|
| 自动生成 | `--cover auto` | 内嵌 SVG 模板 + 书名/作者生成 PNG（离线、无模型） |
| 指定图片 | `--cover <文件>` | 直接用封面图（jpg/png/gif） |
| 不设置 | （省略） | EPUB 无封面 |

`cover.svg` 占位符：

| 占位符 | 说明 |
|--------|------|
| `{{ title_lines }}` | 书名（自动切行，最多 3 行，每行 ≤14 字符） |
| `{{ author }}` | 作者 |

可自定义：背景渐变（`linearGradient`）、装饰线/点、字体、配色。生成的封面固定 1600×2560（2:3.2），同时产出 `cover.xhtml` 封面页并置于 spine 首位，兼容 Kindle 等阅读器。

## 失败与边界

- **字体依赖设备**：工具不自带字体，跨设备（Kindle/iOS/Android）渲染可能不同；无中文字体的设备回退到通用字体。
- **阅读器兼容性**：`max-width`、`margin`、`padding` 等布局属性在部分阅读器中可能被忽略；正文排版（缩进/对齐/pre-line）兼容性最好。
- **预览页是快照**：`preview.html` 是生成时注入 CSS 的静态快照，编辑 `style.css` 后需重新注入/导出才能看到新效果。
- **SVG 语法**：`cover.svg` 是 XML，注释内不允许出现 `--` 序列（会解析失败）。
- **封面渲染依赖系统中文字体**：按 font-family 匹配 PingFang / Microsoft YaHei / Noto Sans CJK SC 等。
- **自定义 cover.svg 接线暂未开放**：当前 `--cover auto` 使用内嵌默认模板；以自定义 `cover.svg` 作为生成源（`--cover-template`）在后续版本规划中。

## 相关链接

- [cli](../cli/) — `--style` / `--cover` 参数用法
- [delivery](../delivery/) — 渲染产物如何打包进 EPUB
- [architecture](../../architecture.md) — 渲染在数据流中的位置
