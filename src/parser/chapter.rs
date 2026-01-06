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
            
            // 先转义HTML特殊字符（只转义文本内容，不影响HTML标签）
            let mut escaped_line = String::new();
            let mut in_tag = false;
            let mut chars = processed_line.chars().peekable();
            
            while let Some(c) = chars.next() {
                // 过滤无效的XML字符
                if c.is_control() && !matches!(c, '\n' | '\r' | '\t') {
                    continue;
                }
                
                if in_tag {
                    // 在标签内，直接添加字符
                    escaped_line.push(c);
                    // 检查是否结束标签
                    if c == '>' {
                        in_tag = false;
                    }
                } else {
                    match c {
                        '<' => {
                            // 检查是否是HTML标签的开始
                            if let Some(next_char) = chars.peek() {
                                if next_char.is_alphabetic() || *next_char == '/' {
                                    // 是HTML标签，不转义
                                    escaped_line.push(c);
                                    in_tag = true;
                                } else {
                                    // 不是HTML标签，转义
                                    escaped_line.push_str("&lt;");
                                }
                            } else {
                                // 行尾的<，转义
                                escaped_line.push_str("&lt;");
                            }
                        }
                        '>' => {
                            // 转义>为&gt;
                            escaped_line.push_str("&gt;");
                        }
                        '&' => {
                            // 转义&为&amp;
                            escaped_line.push_str("&amp;");
                        }
                        _ => {
                            // 其他字符直接添加
                            escaped_line.push(c);
                        }
                    }
                }
            }
            
            // 修复错误格式的HTML标签
            let mut fixed_line = escaped_line.clone();
            
            // 1. 修复 <ahref...> 这种标签名和属性之间缺少空格的情况
            let re_tag_space = Regex::new(r#"<([a-zA-Z][a-zA-Z0-9]*)(href|src|target|alt|width|height|title|class|id|bl_id|a_id|b_id)"#).unwrap();
            fixed_line = re_tag_space.replace_all(&fixed_line, r#"<$1 $2"#).to_string();
            
            // 2. 修复 hrefhttp:// 这种缺少等号和引号的情况
            let re_href = Regex::new(r#"href(http[s]?://[^">]+)"#).unwrap();
            fixed_line = re_href.replace_all(&fixed_line, r#"href="$1""#).to_string();
            
            // 3. 修复 srchttp:// 这种缺少等号和引号的情况
            let re_src = Regex::new(r#"src(http[s]?://[^">]+)"#).unwrap();
            fixed_line = re_src.replace_all(&fixed_line, r#"src="$1""#).to_string();
            
            // 4. 修复 target_blank 这种缺少等号和引号的情况，保留下划线
            let re_attr_underscore = Regex::new(r#"(target|alt|title|class|id|bl_id|a_id|b_id|width|height)_([^\s">]+)"#).unwrap();
            fixed_line = re_attr_underscore.replace_all(&fixed_line, r#"$1="$2""#).to_string();
            
            // 5. 修复 bl_id1234 这种缺少等号和引号的情况
            let re_attr_num = Regex::new(r#"(bl_id|a_id|b_id|width|height)(\d+)"#).unwrap();
            fixed_line = re_attr_num.replace_all(&fixed_line, r#"$1="$2""#).to_string();
            
            // 11. 修复属性之间缺少空格的问题（如 target="_blank"href="..."）
            let re_attr_no_space = Regex::new(r#""([a-zA-Z][a-zA-Z0-9_]*)="#).unwrap();
            fixed_line = re_attr_no_space.replace_all(&fixed_line, r#"" $1="#).to_string();
            
            // 12. 清理属性值前面的多余空格（如 href=" http://）
            let re_attr_leading_space = Regex::new(r#"="\s+"#).unwrap();
            fixed_line = re_attr_leading_space.replace_all(&fixed_line, r#"="#).to_string();
            
            // 13. 清理多余的空格
            let re_extra_space = Regex::new(r#"\s+>"#).unwrap();
            fixed_line = re_extra_space.replace_all(&fixed_line, r#">"#).to_string();
            
            // 14. 修复引号之间的多余空格
            let re_quote_space = Regex::new(r#""\s+""#).unwrap();
            fixed_line = re_quote_space.replace_all(&fixed_line, r#"""#).to_string();
            
            current_paragraph.push_str(&fixed_line);
        }
    }
    
    // 添加最后一个段落（如果非空）
    if !current_paragraph.is_empty() {
        paragraphs.push(current_paragraph.trim().to_string());
    }
    
    paragraphs
}
