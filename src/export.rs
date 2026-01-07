use anyhow::{Context, Result};
use console::style;
use rust_i18n::t;
use std::path::PathBuf;

/// 导出样式模板到指定目录
pub fn export_template(export_dir: &PathBuf) -> Result<()> {
    use std::fs;

    fs::create_dir_all(export_dir)
        .with_context(|| format!("Failed to create export directory: {}", export_dir.display()))?;

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

    let readme_path = export_dir.join("README.md");
    let readme_content = generate_readme();
    fs::write(&readme_path, readme_content)
        .with_context(|| format!("Failed to write README file: {}", readme_path.display()))?;

    println!("{}", style(t!("export-template-success")).green().bold());
    println!("{}", t!("export-template-info"));
    println!("{}", t!("export-template-css"));
    println!("{}", t!("export-template-preview"));
    println!("{}", t!("export-template-readme"));
    println!("{} {}", t!("label-output"), style(export_dir.display()).blue());

    Ok(())
}

/// 生成预览HTML内容
pub fn generate_preview_html(css: &str) -> String {
    let html_template = r##"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BookSmith Preview</title>
    <style>
CSS_PLACEHOLDER
    </style>
</head>
<body>
    <div class="container">
        <h1 class="chapter-title">小说片段：古老图书馆的秘密</h1>
        
        <p class="chapter-content">这是一个宁静的夜晚，月光如水般洒在古老的小镇上。街道两旁的梧桐树影在微风中轻轻摇曳，远处传来隐约的钟声。在小镇的尽头，一座古老的图书馆静静地矗立着，仿佛在诉说着岁月的故事。</p>
        
        <p class="chapter-content">年轻的学者李明推开门，一股淡淡的书香扑面而来。他熟悉这里的每一个角落，每一本书都像是他的老朋友。今夜，他要寻找一本关于古代文明的古籍，那是他研究的关键。</p>
        
        <p class="chapter-content">穿过幽暗的走廊，他来到了古籍区。月光透过彩色玻璃洒在书架上，形成斑驳的光影。突然，一本金色封面的书引起了他的注意。那本书似乎在黑暗中散发着微弱的光芒，仿佛在召唤着他。</p>
        
        <p class="chapter-content">李明伸出手，轻轻取下那本书。封面上传来古老皮革的质感，上面烫金的文字已经有些模糊，但依然可以辨认出那是古埃及文字。他心跳加速，意识到这可能是他一直在寻找的失落文明的记录。</p>
        
        <p class="chapter-content">就在他翻开书的瞬间，一道金光从书中射出，将整个图书馆照亮。李明感到一阵眩晕，当他再次睁开眼睛时，发现自己已经置身于一个陌生的世界...</p>
        
        <h1 class="chapter-title">散文：雨的味道</h1>
        
        <p class="chapter-content">雨，是大自然最温柔的馈赠。它在清晨的窗台上轻轻敲打着，像是一首优美的乐章。我喜欢在这样的日子里，坐在窗前，听着雨声，任思绪飘向远方。</p>
        
        <p class="chapter-content">雨是有味道的。春雨带着泥土的芬芳，夏雨带着青草的气息，秋雨带着果实的香甜，冬雨带着雪花的清冽。每一种雨都有它独特的味道，让人回味无穷。</p>
        
        <p class="chapter-content">雨是有情感的。它有时温柔细腻，有时磅礴大气，有时缠绵悱恻，有时干脆利落。它像是大自然的情绪表达，将各种情感展现得淋漓尽致。</p>
        
        <h1 class="chapter-title">科技文章：人工智能的未来</h1>
        
        <p class="chapter-content">人工智能已经成为当今科技领域最热门的话题之一。从自动驾驶汽车到智能助手，从医疗诊断到金融分析，AI正在改变着我们生活的方方面面。</p>
        
        <p class="chapter-content">深度学习是AI领域的核心技术之一。它模拟人类大脑的神经网络结构，通过大量数据的训练，使计算机能够学习和识别复杂的模式。这种技术已经在图像识别、自然语言处理等领域取得了突破性的进展。</p>
        
        <nav id="toc">
            <h1>目录</h1>
            <ol>
                <li><a href="#">小说片段：古老图书馆的秘密</a></li>
                <li><a href="#">散文：雨的味道</a></li>
                <li><a href="#">科技文章：人工智能的未来</a></li>
            </ol>
        </nav>
    </div>
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

Open `preview.html` in a browser to preview the style effect. The preview includes:
- Novel excerpt
- Essay
- Tech article

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
