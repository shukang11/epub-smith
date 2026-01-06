pub mod encoding;
pub mod chapter;

use clap::Parser;
use std::{fs, path::PathBuf};
use anyhow::{Context, Result};

use crate::{config::Config, models::{Book, Meta}};
use self::{encoding::convert_to_utf8, chapter::parse_chapters};

/// 将TXT文件解析为Book结构
pub fn parse_txt(input: &PathBuf, config: &Config) -> Result<Book> {
    // 读取文件内容
    let read_start = std::time::Instant::now();
    let raw_content = fs::read(input)
        .with_context(|| format!("Failed to read input file: {}", input.display()))?;
    let read_duration = read_start.elapsed();

    // 转换为UTF-8字符串
    let encoding_start = std::time::Instant::now();
    let content = convert_to_utf8(&raw_content, config.encoding.as_deref());
    let encoding_duration = encoding_start.elapsed();

    let lines_start = std::time::Instant::now();
    let lines: Vec<&str> = content.lines().collect();
    let lines_duration = lines_start.elapsed();

    // 解析章节
    let parse_start = std::time::Instant::now();
    let chapters = parse_chapters(&lines, &config.rules, config.explain)?;
    let parse_duration = parse_start.elapsed();

    // 从文件名生成默认标题
    let title_start = std::time::Instant::now();
    let default_title = input.file_stem()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("Untitled");

    // 直接使用文件名作为书名，更可靠
    let title = default_title.to_string();
    let author = "Unknown".to_string();

    // 创建默认元数据
    let meta = Meta {
        title,
        author,
        language: "zh-CN".to_string(),
        identifier: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
        modified: chrono::Utc::now().to_rfc3339(),
        cover: None,
    };
    let title_duration = title_start.elapsed();

    // 如果是debug模式，打印详细计时信息
    if let Ok(args) = crate::cli::Args::try_parse_from(std::env::args_os()) {
        if args.debug {
            eprintln!("  ┌─────────────────────────────────────────────────────────────────────────────");
            eprintln!("  │ Text Parsing Details");
            eprintln!("  ├───────────────────────────────────┬───────────────────────────────────────");
            eprintln!("  │ Step                              │ Duration                              ");
            eprintln!("  ├───────────────────────────────────┼───────────────────────────────────────");
            eprintln!("  │ File reading                      │ {:<37.2?}", read_duration);
            eprintln!("  │ Encoding conversion               │ {:<37.2?}", encoding_duration);
            eprintln!("  │ Line splitting                    │ {:<37.2?}", lines_duration);
            eprintln!("  │ Chapter parsing                   │ {:<37.2?}", parse_duration);
            eprintln!("  │ Metadata creation                 │ {:<37.2?}", title_duration);
            eprintln!("  └───────────────────────────────────┴───────────────────────────────────────");
        }
    }

    Ok(Book {
        meta,
        chapters,
    })
}
