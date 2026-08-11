# 模板与样式参考

## 默认样式方案

默认 `style.css`（方案 C，可读性优先）：
- 正文：首行缩进 `2em`、段后距 `0.95em`、两端对齐、`white-space: pre-line` 保留段内换行
- 正文基准：衬线字体族、18px、行高 1.8
- 章节标题：居中、深色（默认 CSS 为 1.45em + 全宽灰线；可自定义，见下）

## 样式自定义（style.css）

`template export` 导出 `style.css`，它控制 EPUB 中所有页面的视觉样式。编辑后用 `--style <文件>` 让转换/预览生效：

```bash
epub-smith convert novel.txt --style my_tpl/style.css -o out.epub
```

### 可自定义项

| 选择器 | 控制内容 | 说明 |
|--------|----------|------|
| `body` | 全局字体族、字号、行高、文字/背景色 | 中文字体族按设备回退（宋体/雅黑/Noto 等） |
| `.container` | 章节/目录页内容容器：max-width、居中、内边距 | 宽屏/浏览器预览时内容居中留白 |
| `h1.chapter-title` | 章节标题：字号、字重、对齐、字距、颜色 | 可用 `::after` 做装饰线 |
| `p.chapter-content` | 正文段落：首行缩进、段距、对齐、段内换行 | `white-space: pre-line` 保留段内换行 |
| `.cover-page` | 封面页整体：居中、背景色 | 默认白底 |
| `.cover-image` | 封面图：自适应宽度、不拉伸变形 | `max-width:100%; height:auto` |
| `nav#toc` | 目录导航：标题、列表、链接样式 | 仅目录页 |
| `@media print` | 打印/分页行为 | `page-break-before: always` 每章分页 |

### 页面结构

```
章节页：<body> > div.container > (h1.chapter-title + p.chapter-content)
目录页：<body> > div.container > nav#toc
封面页：<body.cover-page> > img.cover-image
```

章节与目录页共用 `.container`（内容居中留白），封面页独立用 `.cover-page`（居中显示封面图）。

### 示例：定制标题（金色短装饰线，呼应封面）

```css
h1.chapter-title {
    font-size: 1.5em;
    font-weight: 600;
    text-align: center;
    letter-spacing: 0.08em;
    color: #2e4057;
    margin: 2em 0 1.4em 0;
    padding-bottom: 0.6em;
    position: relative;
}
h1.chapter-title::after {
    content: "";
    display: block;
    width: 3em;
    height: 3px;
    margin: 0.55em auto 0;
    background: #c9a86a;
    border-radius: 2px;
}
```

### 边界与上限

- **字体依赖设备**：工具不自带字体，跨设备（Kindle/iOS/Android）渲染字体可能不同；无中文字体的设备会回退到通用字体。
- **EPUB 阅读器兼容性**：`max-width`、`margin`、`padding` 等布局属性在部分阅读器中可能被忽略；正文排版（缩进/对齐/pre-line）兼容性最好。
- **预览页是快照**：`preview.html` 是生成时把 CSS 注入的静态快照，编辑 `style.css` 后需重新注入/导出才能看到新效果。

## 预览页（preview.html）

`template export` 导出 `preview.html`，用于在浏览器中预览 `style.css` 的样式效果。

- 顶部目录（固定最上方）+ 翻页控件（◀/▶、键盘 ←→）+ 示例封面 + 6 个示例章节（小说/散文/科技/超长标题/对话/短段落）
- 内嵌 `style.css`，编辑后重新注入并刷新即可对照效果

## 封面设置

### 三种来源

| 方式 | 命令 | 说明 |
|------|------|------|
| 自动生成 | `--cover auto` | 用内嵌 SVG 模板 + 书名/作者生成 PNG（离线、无模型） |
| 指定图片 | `--cover <文件>` | 直接用你的封面图（jpg/png/gif） |
| 不设置 | （省略） | EPUB 无封面 |

### 封面模板（cover.svg）

`template export` 导出 `cover.svg`。占位符：

| 占位符 | 说明 |
|--------|------|
| `{{ title_lines }}` | 书名（自动切行，最多 3 行，每行 ≤14 字符） |
| `{{ author }}` | 作者 |

可自定义：背景渐变（`linearGradient`）、装饰线/点、字体、配色。

### 封面边界

- 当前 `--cover auto` 使用**内嵌默认模板**；自定义 `cover.svg` 作为生成源的接线（`--cover-template`）待后续版本开放。
- SVG 为 XML，注释内**不允许出现 `--` 序列**（会解析失败）。
- 封面渲染依赖系统中文字体（PingFang / Microsoft YaHei / Noto Sans CJK SC 等，按 font-family 匹配）。
- 封面尺寸固定 1600×2560（2:3.2），写入 EPUB 后按阅读器显示。
- 生成的封面会同时产出 `cover.xhtml` 封面页并置于 spine 首位，兼容 Kindle 等阅读器。
