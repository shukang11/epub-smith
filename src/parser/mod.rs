pub mod chapter;
pub mod encoding;

use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use self::{chapter::parse_chapters, encoding::convert_to_utf8};
use crate::{
    config::Config,
    models::{Book, Meta},
};

/// 读取单个输入源的内容
fn read_input_source(input: &PathBuf, encoding: Option<&str>) -> Result<String> {
    let content = if input.to_string_lossy() == "-" {
        // 从标准输入读取
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        // 从文件读取
        let raw_content = fs::read(input)
            .with_context(|| format!("Failed to read input file: {}", input.display()))?;
        convert_to_utf8(&raw_content, encoding)
    };
    Ok(content)
}

/// 将多个TXT文件解析为Book结构
pub fn parse_txt(inputs: &[PathBuf], config: &Config) -> Result<Book> {
    // 开始总解析计时
    let total_parse_start = std::time::Instant::now();

    let mut all_lines = Vec::new();

    // 读取并合并所有输入源的内容
    for input in inputs {
        let content = read_input_source(input, config.encoding.as_deref())?;
        all_lines.extend(content.lines().map(|s| s.to_string()));
    }

    // 转换为&str切片
    let lines: Vec<&str> = all_lines.iter().map(|s| s.as_str()).collect();

    // 解析章节
    let chapters = parse_chapters(&lines, &config.rules, config.explain)?;

    // 生成默认标题
    let default_title = if inputs.is_empty() {
        "Untitled"
    } else {
        inputs[0]
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("Untitled")
    };

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

    // 如果是debug模式，打印详细计时信息
    if let Ok(args) = crate::cli::Args::try_parse_from(std::env::args_os())
        && args.debug
    {
        let total_parse_duration = total_parse_start.elapsed();

        // 使用eprintln!打印调试信息，避免类型转换问题
        let total_parse_duration_str = format!("{:.2?}", total_parse_duration);
        let input_files_str = format!("{}", inputs.len());
        let total_lines_str = format!("{}", lines.len());
        let chapters_detected_str = format!("{}", chapters.len());

        eprintln!(
            "  ┌─────────────────────────────────────────────────────────────────────────────"
        );
        eprintln!("  │ Text Parsing Details");
        eprintln!("  ├───────────────────────────────────┬───────────────────────────────────────");
        eprintln!("  │ Step                              │ Duration                              ");
        eprintln!("  ├───────────────────────────────────┼───────────────────────────────────────");
        eprintln!(
            "  │ Total parsing time                │ {:<37}",
            total_parse_duration_str
        );
        eprintln!(
            "  │ Input files processed             │ {:<37}",
            input_files_str
        );
        eprintln!(
            "  │ Total lines processed             │ {:<37}",
            total_lines_str
        );
        eprintln!(
            "  │ Chapters detected                 │ {:<37}",
            chapters_detected_str
        );
        eprintln!("  └───────────────────────────────────┴───────────────────────────────────────");
    }

    Ok(Book { meta, chapters })
}
