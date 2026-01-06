use std::path::PathBuf;
use anyhow::Result;
use booksmith::{parser::parse_txt, config::Config, cli::Args};
use tempfile::NamedTempFile;

#[test]
fn test_parse_txt() -> Result<()> {
    // Create sample content
    let content = r#"第1章 开端
这是第一章的内容

第2章 发展
这是第二章的内容

第3章 高潮
这是第三章的内容

第4章 结局
这是第四章的内容

番外 后续
这是番外的内容"#;
    
    // Create temporary file
    let temp_file = NamedTempFile::new()?;
    std::fs::write(temp_file.path(), content)?;
    let input = temp_file.path().to_owned();
    
    // Create default args and config
    let args = Args {
        input: Some(input.clone()),
        rules: None,
        output: PathBuf::from("output.epub"),
        encoding: None,
        dry_run: false,
        print_outline: false,
        explain: false,
        title: None,
        author: None,
        cover: None,
        language: "zh-CN".to_string(),
        check: false,
        verbose: false,
        lang: None,
        export_template: None,
        style: None,
    };
    
    let config = Config::from_args(&args)?;
    
    // Parse the file
    let book = parse_txt(&input, &config)?;
    
    // Verify the result
    assert_eq!(book.chapters.len(), 5);
    assert_eq!(book.chapters[0].title, "第1章 开端");
    assert_eq!(book.chapters[1].title, "第2章 发展");
    assert_eq!(book.chapters[2].title, "第3章 高潮");
    assert_eq!(book.chapters[3].title, "第4章 结局");
    assert_eq!(book.chapters[4].title, "番外 后续");
    
    Ok(())
}

#[test]
fn test_parse_txt_with_broken_html() -> Result<()> {
    // Create sample content with various broken HTML tags and special characters
    let content = r#"第1章 开端
这是第一章的内容，包含错误格式的链接：
<ahrefhttp://www.example.com/showbook.asp?bl_id140700target_blank>点击查看</a>

第2章 发展
这是第二章的内容，包含另一个错误链接和特殊字符：
<ahrefhttps://www.test.com/mybook_votebook.asp?a_id11877676&b_id136657target_blank>投票支持</a>
文本中的特殊字符：& < > 

第3章 图片示例
这是第三章的内容，包含错误格式的图片标签：
<img srchttp://www.example.com/image.jpgalt测试图片width200height150>

第4章 其他标签
这是第四章的内容，包含其他错误格式的标签：
<divclasscontaineridmain>
<pclassparagraph>这是一个段落</p>
<spanclasshighlight>高亮文本</span>
</div>

第5章 标题标签
这是第五章的内容，包含错误格式的标题标签：
<h1title主标题>主标题</h1>
<h2title副标题>副标题</h2>"#;
    
    // Create temporary file
    let temp_file = NamedTempFile::new()?;
    std::fs::write(temp_file.path(), content)?;
    let input = temp_file.path().to_owned();
    
    // Create default args and config
    let args = Args {
        input: Some(input.clone()),
        rules: None,
        output: PathBuf::from("output.epub"),
        encoding: None,
        dry_run: false,
        print_outline: false,
        explain: false,
        title: None,
        author: None,
        cover: None,
        language: "zh-CN".to_string(),
        check: false,
        verbose: false,
        lang: None,
        export_template: None,
        style: None,
    };
    
    let config = Config::from_args(&args)?;
    
    // Parse the file
    let book = parse_txt(&input, &config)?;
    
    // Verify the result
    assert_eq!(book.chapters.len(), 5);
    
    // Check chapter 1: a tag with http link
    let chapter1_paragraphs = &book.chapters[0].paragraphs;
    assert!(!chapter1_paragraphs.is_empty());
    
    // 调试输出：打印所有段落内容
    println!("Chapter 1 paragraphs:");
    for (i, p) in chapter1_paragraphs.iter().enumerate() {
        println!("  Paragraph {}: {}", i + 1, p);
    }
    
    let chapter1_has_fixed_link = chapter1_paragraphs
        .iter()
        .any(|p| p.contains(r#"href="http://www.example.com"#));
    assert!(chapter1_has_fixed_link, "Chapter 1 should contain fixed http link");
    
    // Check chapter 2: a tag with https link and special characters
    let chapter2_paragraphs = &book.chapters[1].paragraphs;
    assert!(!chapter2_paragraphs.is_empty());
    
    let chapter2_has_fixed_link = chapter2_paragraphs
        .iter()
        .any(|p| p.contains(r#"href="https://www.test.com"#));
    assert!(chapter2_has_fixed_link, "Chapter 2 should contain fixed https link");
    
    let chapter2_has_escaped_chars = chapter2_paragraphs
        .iter()
        .any(|p| p.contains("&amp;") && p.contains("&lt;") && p.contains("&gt;"));
    assert!(chapter2_has_escaped_chars, "Chapter 2 should contain escaped special characters");
    
    // Check chapter 3: img tag with src attribute
    let chapter3_paragraphs = &book.chapters[2].paragraphs;
    assert!(!chapter3_paragraphs.is_empty());
    
    // 调试输出：打印第3章段落内容
    println!("Chapter 3 paragraphs:");
    for (i, p) in chapter3_paragraphs.iter().enumerate() {
        println!("  Paragraph {}: {}", i + 1, p);
    }
    
    let chapter3_has_fixed_img = chapter3_paragraphs
        .iter()
        .any(|p| p.contains(r#"src="http://www.example.com"#));
    assert!(chapter3_has_fixed_img, "Chapter 3 should contain fixed img tag");
    
    // Check chapter 4: div, p, span tags with attributes
    let chapter4_paragraphs = &book.chapters[3].paragraphs;
    assert!(!chapter4_paragraphs.is_empty());
    
    let chapter4_has_div = chapter4_paragraphs
        .iter()
        .any(|p| p.contains(r#"<div"#));
    assert!(chapter4_has_div, "Chapter 4 should contain div tag");
    
    // Check chapter 5: h1, h2 tags with attributes
    let chapter5_paragraphs = &book.chapters[4].paragraphs;
    assert!(!chapter5_paragraphs.is_empty());
    
    let chapter5_has_h1 = chapter5_paragraphs
        .iter()
        .any(|p| p.contains(r#"<h1"#));
    assert!(chapter5_has_h1, "Chapter 5 should contain h1 tag");
    
    Ok(())
}
