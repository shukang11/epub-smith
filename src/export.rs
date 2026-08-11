use crate::output::GLOBAL_OUTPUT;
use anyhow::{Context, Result};
use rust_i18n::t;
use std::path::PathBuf;

/// 导出样式模板到指定目录
pub fn export_template(export_dir: &PathBuf) -> Result<()> {
    use std::fs;

    fs::create_dir_all(export_dir).with_context(|| {
        format!(
            "Failed to create export directory: {}",
            export_dir.display()
        )
    })?;

    let css_path = export_dir.join("style.css");
    let css_content = include_str!("resources/css/default.css");
    fs::write(&css_path, css_content)
        .with_context(|| format!("Failed to write CSS file: {}", css_path.display()))?;

    // 读取刚刚写入的CSS内容（确保使用最新的CSS，包括用户可能修改过的）
    let css_content = fs::read_to_string(&css_path)
        .with_context(|| format!("Failed to read CSS file: {}", css_path.display()))?;

    let preview_path = export_dir.join("preview.html");
    let preview_content = generate_preview_html(&css_content);
    fs::write(&preview_path, &preview_content)
        .with_context(|| format!("Failed to write preview file: {}", preview_path.display()))?;

    // 导出封面模板（--cover auto 使用）
    let cover_svg_path = export_dir.join("cover.svg");
    let cover_svg_content = include_str!("templates/cover.svg");
    fs::write(&cover_svg_path, cover_svg_content).with_context(|| {
        format!(
            "Failed to write cover SVG file: {}",
            cover_svg_path.display()
        )
    })?;

    let readme_path = export_dir.join("README.md");
    let readme_content = generate_readme();
    fs::write(&readme_path, readme_content)
        .with_context(|| format!("Failed to write README file: {}", readme_path.display()))?;

    GLOBAL_OUTPUT.success(t!("export-template-success"));
    GLOBAL_OUTPUT.info(t!("export-template-info"));
    GLOBAL_OUTPUT.info(t!("export-template-css"));
    GLOBAL_OUTPUT.info(t!("export-template-preview"));
    GLOBAL_OUTPUT.info(t!("export-template-cover"));
    GLOBAL_OUTPUT.info(t!("export-template-readme"));
    GLOBAL_OUTPUT.info(format!("{} {}", t!("label-output"), export_dir.display()));

    Ok(())
}

/// 生成预览HTML内容（含示例封面、目录与翻页交互）
pub fn generate_preview_html(css: &str) -> String {
    let html_template = r##"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BookSmith Preview</title>
    <style>
CSS_PLACEHOLDER

/* ===== 预览页附加样式（仅预览交互用，不进入 EPUB） ===== */
.preview-shell { max-width: 900px; margin: 0 auto; padding: 16px; }
.preview-cover {
    align-items: center; justify-content: center;
    background: linear-gradient(160deg, #1a2634, #2e4057);
    border: 3px solid #c9a86a; border-radius: 6px;
    padding: 48px 24px; margin-bottom: 20px; color: #f5f0e6; text-align: center;
    display: none;
}
.preview-cover.active { display: flex; }
.preview-cover .cover-title { font-size: 2em; font-weight: bold; letter-spacing: 0.2em; }
.preview-cover .cover-author { margin-top: 16px; color: #c9a86a; font-size: 1em; }
#preview-toc { position: sticky; top: 0; background: #ffffff; border-bottom: 1px solid #eeeeee; padding: 8px 0; z-index: 10; text-align: center; }
#preview-toc a { margin-right: 10px; color: #2e4057; text-decoration: none; cursor: pointer; }
#preview-toc a:hover, #preview-toc a.active { color: #c9a86a; text-decoration: underline; }
#preview-controls { display: flex; align-items: center; justify-content: center; gap: 8px; padding: 10px 0; border-bottom: 1px solid #eeeeee; }
#preview-controls button { padding: 4px 14px; cursor: pointer; }
#preview-pos { margin: 0 8px; color: #666; font-size: 0.9em; }
.chapter-view { display: none; }
.chapter-view.active { display: block; }
    </style>
</head>
<body>
<div class="preview-shell">
    <!-- 目录：固定最上方 -->
    <nav id="preview-toc">
        <strong>目录：</strong>
        <a data-idx="0">封面</a>
        <a data-idx="1">小说片段</a>
        <a data-idx="2">散文</a>
        <a data-idx="3">科技文章</a>
        <a data-idx="4">长标题示例</a>
        <a data-idx="5">对话示例</a>
        <a data-idx="6">短段示例</a>
    </nav>

    <!-- 翻页控件：正文上方 -->
    <div id="preview-controls">
        <button onclick="prevChapter()">◀ 上一章</button>
        <span id="preview-pos">1 / 7</span>
        <button onclick="nextChapter()">下一章 ▶</button>
    </div>

    <!-- 内容区：封面 + 章节，切换显示 -->
    <div class="container" id="preview-content">
        <!-- 封面（第 0 页） -->
        <div class="preview-cover" id="preview-cover">
            <div>
                <div class="cover-title">示例书籍</div>
                <div class="cover-author">作者：佚名</div>
            </div>
        </div>
        <section class="chapter-view active">
            <h1 class="chapter-title">小说片段：古老图书馆的秘密</h1>
            <p class="chapter-content">这是一个宁静的夜晚，月光如水般洒在古老的小镇上。街道两旁的梧桐树影在微风中轻轻摇曳，远处传来隐约的钟声。在小镇的尽头，一座古老的图书馆静静地矗立着，仿佛在诉说着岁月的故事。</p>
            <p class="chapter-content">年轻的学者李明推开门，一股淡淡的书香扑面而来。他熟悉这里的每一个角落，每一本书都像是他的老朋友。今夜，他要寻找一本关于古代文明的古籍，那是他研究的关键。</p>
            <p class="chapter-content">穿过幽暗的走廊，他来到了古籍区。月光透过彩色玻璃洒在书架上，形成斑驳的光影。突然，一本金色封面的书引起了他的注意。那本书似乎在黑暗中散发着微弱的光芒，仿佛在召唤着他。</p>
            <p class="chapter-content">李明伸出手，轻轻取下那本书。封面上传来古老皮革的质感，上面烫金的文字已经有些模糊，但依然可以辨认出那是古埃及文字。他心跳加速，意识到这可能是他一直在寻找的失落文明的记录。</p>
            <p class="chapter-content">就在他翻开书的瞬间，一道金光从书中射出，将整个图书馆照亮。李明感到一阵眩晕，当他再次睁开眼睛时，发现自己已经置身于一个陌生的世界……</p>
        </section>

        <section class="chapter-view">
            <h1 class="chapter-title">散文：雨的味道</h1>
            <p class="chapter-content">雨，是大自然最温柔的馈赠。它在清晨的窗台上轻轻敲打着，像是一首优美的乐章。我喜欢在这样的日子里，坐在窗前，听着雨声，任思绪飘向远方。</p>
            <p class="chapter-content">雨是有味道的。春雨带着泥土的芬芳，夏雨带着青草的气息，秋雨带着果实的香甜，冬雨带着雪花的清冽。每一种雨都有它独特的味道，让人回味无穷。</p>
            <p class="chapter-content">雨是有情感的。它有时温柔细腻，有时磅礴大气，有时缠绵悱恻，有时干脆利落。它像是大自然的情绪表达，将各种情感展现得淋漓尽致。</p>
        </section>

        <section class="chapter-view">
            <h1 class="chapter-title">科技文章：人工智能的未来</h1>
            <p class="chapter-content">人工智能已经成为当今科技领域最热门的话题之一。从自动驾驶汽车到智能助手，从医疗诊断到金融分析，AI正在改变着我们生活的方方面面。</p>
            <p class="chapter-content">深度学习是AI领域的核心技术之一。它模拟人类大脑的神经网络结构，通过大量数据的训练，使计算机能够学习和识别复杂的模式。这种技术已经在图像识别、自然语言处理等领域取得了突破性的进展。</p>
            <p class="chapter-content">然而，人工智能的发展也伴随着对就业、隐私与伦理的深刻讨论。如何在技术进步与社会责任之间取得平衡，将成为未来十年最重要的议题之一。</p>
        </section>

        <section class="chapter-view">
            <h1 class="chapter-title">特别长的小说章节标题，用于检验在窄屏设备上的换行与排版是否自然美观</h1>
            <p class="chapter-content">本章节用于检验超长标题在阅读器中的表现。当章节标题很长时，应当能够在适当的位置换行，而不是被生硬地截断或撑破版面。</p>
            <p class="chapter-content">同时，这也用于检验标题与正文之间、标题之间的间距是否协调，确保长标题不会影响后续内容的可读性。</p>
        </section>

        <section class="chapter-view">
            <h1 class="chapter-title">对话示例</h1>
            <p class="chapter-content">“你确定要这么做吗？”她轻声问道，目光中带着一丝忧虑。</p>
            <p class="chapter-content">“这是唯一的选择。”他握紧拳头，声音低沉而坚定，“如果现在退缩，之前所有的努力都会付诸东流。”</p>
            <p class="chapter-content">窗外的夜色渐深，两人在昏暗的灯光下沉默良久。最终，她点了点头：“那就……一起走下去吧。”</p>
        </section>

        <section class="chapter-view">
            <h1 class="chapter-title">短段落示例</h1>
            <p class="chapter-content">风。</p>
            <p class="chapter-content">从山那边吹过来。</p>
            <p class="chapter-content">带着松针和泥土的气息。</p>
            <p class="chapter-content">他站在崖边，衣角被风掀起，仿佛下一秒就要被卷走。</p>
            <p class="chapter-content">但他没有动。</p>
        </section>
    </div>
</div>
<script>
(function () {
    var views = [document.getElementById('preview-cover')]
        .concat(Array.prototype.slice.call(document.querySelectorAll('.chapter-view')));
    var links = document.querySelectorAll('#preview-toc a');
    var pos = document.getElementById('preview-pos');
    var idx = 0, total = views.length;
    function show(i) {
        idx = (i + total) % total;
        views.forEach(function (v, j) { v.classList.toggle('active', j === idx); });
        links.forEach(function (l, j) { l.classList.toggle('active', j === idx); });
        pos.textContent = (idx + 1) + ' / ' + total;
    }
    function prevChapter() { show(idx - 1); }
    function nextChapter() { show(idx + 1); }
    links.forEach(function (l, j) { l.addEventListener('click', function () { show(j); }); });
    document.addEventListener('keydown', function (e) {
        if (e.key === 'ArrowLeft') { show(idx - 1); }
        else if (e.key === 'ArrowRight') { show(idx + 1); }
    });
    window.prevChapter = prevChapter;
    window.nextChapter = nextChapter;
    show(0);
})();
</script>
</body>
</html>
"##;
    html_template.replace("CSS_PLACEHOLDER", css)
}

/// 生成README内容
pub fn generate_readme() -> String {
    let content = r#"# BookSmith Style Template

This directory contains BookSmith's style template files for customizing EPUB appearance.

## File Description

- `style.css` - Style file defining fonts, sizes, colors and visual effects
- `preview.html` - Preview file, can be opened in browser
- `cover.svg` - Default cover template (used by `--cover auto`), editable
- `README.md` - This instruction file

## Usage

### 1. Edit Styles

Open `style.css` and modify the style properties. Common customizable items:

```css
/* Font settings */
body {
    font-family: "SimSun", "Songti SC", serif;
    font-size: 16px;
}

/* Chapter title */
h1.chapter-title {
    font-size: 24px;
    text-align: center;
}

/* Paragraph */
p.chapter-content {
    text-indent: 2em;
    text-align: justify;
}
```

### 2. Preview Effect

Open `preview.html` in a browser to preview the style effect. The preview page includes:
- A sample cover preview (styling reference)
- 6 sample chapters covering different layouts: novel excerpt, essay, tech article, long title, dialogue, short paragraphs
- A table of contents and prev/next chapter controls — click a TOC entry, use the ◀/▶ buttons, or press the ← / → arrow keys to switch chapters

### 3. Generate EPUB

Use `--style` parameter to specify custom style file:

```bash
booksmith your_book.txt --style ./mystyle/style.css -o my_book.epub
```

## Notes

- After modifying `style.css`, preview in browser first
- Ensure fonts are available on target devices
- Some styles may not work in all EPUB readers
- If custom styles fail to load, system uses default styles

## Recommended Style Configurations

### Novel Style (Default)
- Serif fonts (SimSun/Songti)
- 16px font size
- 2em text indent
- Justified alignment

### Modern Style
```css
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Microsoft YaHei", sans-serif;
    font-size: 18px;
    line-height: 1.8;
}
```

### Eye Care Mode
```css
body {
    font-family: "Georgia", serif;
    font-size: 18px;
    color: #333333;
    background-color: #f5f5dc;
}
```
"#;
    content.to_string()
}
