pub mod encoding;
pub mod chapter;

use std::{fs, path::PathBuf};
use anyhow::{Context, Result};

use crate::{config::Config, models::{Book, Meta}};
use self::{encoding::convert_to_utf8, chapter::parse_chapters};

/// 将TXT文件解析为Book结构
pub fn parse_txt(input: &PathBuf, config: &Config) -> Result<Book> {
    // 读取文件内容
    let raw_content = fs::read(input)
        .with_context(|| format!("Failed to read input file: {}", input.display()))?;

    // 转换为UTF-8字符串
    let content = convert_to_utf8(&raw_content, config.encoding.as_deref());
    let lines: Vec<&str> = content.lines().collect();

    // 解析章节
    let chapters = parse_chapters(&lines, &config.rules, config.explain)?;

    // 从文件名生成默认标题
    let default_title = input.file_stem()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("Untitled");

    // 创建默认元数据
    let meta = Meta {
        title: default_title.to_string(),
        author: "Unknown".to_string(),
        language: "zh-CN".to_string(),
        identifier: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
        modified: chrono::Utc::now().to_rfc3339(),
        cover: None,
    };

    Ok(Book {
        meta,
        chapters,
    })
}
