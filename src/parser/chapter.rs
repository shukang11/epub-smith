use anyhow::{Context, Result};
use regex::Regex;
use slug::slugify;

// 初始化国际化支持
rust_i18n::i18n!("locales", fallback = "en");

// 导入t宏用于翻译
use rust_i18n::t;

use crate::models::{Chapter, Rules};

/// 从行列表中解析章节
pub fn parse_chapters(lines: &[&str], rules: &Rules, explain: bool) -> Result<Vec<Chapter>> {
    // 编译所有章节正则表达式
    let regex_patterns: Vec<Regex> = rules.chapter.regex.iter()
        .map(|pattern| Regex::new(pattern)
            .with_context(|| format!("Invalid regex pattern: {}", pattern)))
        .collect::<Result<_>>()?;

    // 查找所有章节起始位置
    let mut chapter_starts: Vec<(usize, &str)> = Vec::new();
    
    if explain {
        println!("{}", t!("explain-using-regex"));
        for (i, pattern) in rules.chapter.regex.iter().enumerate() {
            println!("  {}. {}", i + 1, pattern);
        }
        println!();
        
        println!("{}", t!("explain-detecting-chapters"));
    }
    
    for (line_num, line) in lines.iter().enumerate() {
        // 检查该行是否匹配任何章节模式
        for (i, regex) in regex_patterns.iter().enumerate() {
            if regex.is_match(line) {
                if explain {
                println!("{}", t!("explain-matched-line", line = line_num + 1, index = i + 1, content = line.trim()));
            }
                chapter_starts.push((line_num, line));
                break;
            }
        }
    }

    // 如果没有找到章节，将整个文件视为一个章节
    if chapter_starts.is_empty() {
        let end_line = if lines.is_empty() {
            0
        } else {
            lines.len() - 1
        };
        let chapter = create_chapter(lines, 0, end_line, "Chapter 1", rules)?;
        return Ok(vec![chapter]);
    }

    // 从起始位置创建章节
    let mut chapters = Vec::new();
    
    for (i, (start_line, title)) in chapter_starts.iter().enumerate() {
        let end_line = if i < chapter_starts.len() - 1 {
            chapter_starts[i + 1].0 - 1
        } else {
            lines.len() - 1
        };
        
        let chapter = create_chapter(lines, *start_line, end_line, title, rules)?;
        chapters.push(chapter);
    }

    Ok(chapters)
}

/// 从行范围创建章节
fn create_chapter(
    lines: &[&str], 
    start_line: usize, 
    end_line: usize, 
    raw_title: &str, 
    rules: &Rules
) -> Result<Chapter> {
    // 提取章节内容行，处理空切片的情况
    let content_lines = if lines.is_empty() {
        &[]
    } else {
        &lines[start_line..=end_line]
    };
    
    // 根据规则处理段落
    let paragraphs = process_paragraphs(content_lines, &rules.paragraph);
    
    // 生成章节slug
    let slug = slugify(raw_title);
    
    Ok(Chapter {
        title: raw_title.trim().to_string(),
        paragraphs,
        slug,
        start_line,
        end_line,
    })
}

/// 根据规则处理段落
fn process_paragraphs(lines: &[&str], rules: &crate::models::ParagraphRules) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current_paragraph = String::new();
    
    for line in lines {
        let processed_line = if rules.trim_whitespace {
            line.trim()
        } else {
            line
        };
        
        if processed_line.is_empty() {
            // 如果行是空的，且当前段落非空，将其添加到段落列表
            if !current_paragraph.is_empty() {
                paragraphs.push(current_paragraph.trim().to_string());
                current_paragraph.clear();
            }
        } else {
            // 如果行非空，将其添加到当前段落
            if !current_paragraph.is_empty() {
                if rules.merge_lines {
                    current_paragraph.push(' ');
                } else {
                    current_paragraph.push('\n');
                }
            }
            
            // 修复错误格式的HTML标签
            let fixed_line = processed_line
                // 修复 <ahrefhttp://example.com> 这种错误格式（缺少空格）
                .replace("<a", "<a ")
                // 修复 hrefhttp://example.com 这种错误格式（缺少等号）
                .replace("hrefhttp://", "href=\"http://")
                .replace("hrefhttps://", "href=\"https://")
                // 修复 target_blank 这种错误格式（缺少等号和引号）
                .replace("target_blank", "target=\"_blank\"")
                // 修复 bl_id140700 这种错误格式（缺少等号）
                .replace("bl_id", "bl_id=")
                .replace("a_id", "a_id=")
                .replace("b_id", "b_id=")
                // 修复 URL 末尾缺少引号的问题
                .replace("target=\"_blank\">", ")\" target=\"_blank\">");
            
            current_paragraph.push_str(&fixed_line);
        }
    }
    
    // 添加最后一个段落（如果非空）
    if !current_paragraph.is_empty() {
        paragraphs.push(current_paragraph.trim().to_string());
    }
    
    paragraphs
}
