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
    // Create sample content with broken HTML tags
    let content = r#"第1章 开端
这是第一章的内容，包含错误格式的链接：
<ahrefhttp://www.example.com/showbook.asp?bl_id140700target_blank>点击查看</a>

第2章 发展
这是第二章的内容，包含另一个错误链接：
<ahrefhttps://www.test.com/mybook_votebook.asp?a_id11877676&b_id136657target_blank>投票支持</a>"#;
    
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
    assert_eq!(book.chapters.len(), 2);
    
    // Check chapter 1 paragraphs
    let chapter1_paragraphs = &book.chapters[0].paragraphs;
    assert!(!chapter1_paragraphs.is_empty());
    
    // Check that broken HTML tags are fixed in chapter 1
    let chapter1_has_fixed_link = chapter1_paragraphs
        .iter()
        .any(|p| p.contains(r#"href="http://www.example.com"#) && p.contains(r#"target="_blank""#));
    assert!(chapter1_has_fixed_link, "Chapter 1 should contain fixed link");
    
    // Check chapter 2 paragraphs
    let chapter2_paragraphs = &book.chapters[1].paragraphs;
    assert!(!chapter2_paragraphs.is_empty());
    
    // Check that broken HTML tags are fixed in chapter 2
    let chapter2_has_fixed_link = chapter2_paragraphs
        .iter()
        .any(|p| p.contains(r#"href="https://www.test.com"#) && p.contains(r#"target="_blank""#));
    assert!(chapter2_has_fixed_link, "Chapter 2 should contain fixed link");
    
    Ok(())
}
