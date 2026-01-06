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
        input: input.clone(),
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
