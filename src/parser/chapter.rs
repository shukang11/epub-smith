use anyhow::{Context, Result};
use clap::Parser;
use lazy_static::lazy_static;
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
    let regex_start = std::time::Instant::now();
    let regex_patterns: Vec<Regex> = rules.chapter.regex.iter()
        .map(|pattern| Regex::new(pattern)
            .with_context(|| format!("Invalid regex pattern: {}", pattern)))
        .collect::<Result<_>>()?;
    let regex_duration = regex_start.elapsed();

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
    
    let detect_start = std::time::Instant::now();
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
    let detect_duration = detect_start.elapsed();

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
    let create_start = std::time::Instant::now();
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
    let create_duration = create_start.elapsed();

    // 打印详细计时信息
    if let Ok(args) = crate::cli::Args::try_parse_from(std::env::args_os()) {
        if args.debug {
            eprintln!("  ┌─────────────────────────────────────────────────────────────────────────────");
            eprintln!("  │ Chapter Parsing Details");
            eprintln!("  ├───────────────────────────────────┬───────────────────────────────────────");
            eprintln!("  │ Step                              │ Duration                              ");
            eprintln!("  ├───────────────────────────────────┼───────────────────────────────────────");
            eprintln!("  │ Regex compilation                 │ {:<37.2?}", regex_duration);
            eprintln!("  │ Chapter detection                 │ {:<37.2?}", detect_duration);
            eprintln!("  │ Chapter creation                  │ {:<37.2?}", create_duration);
            eprintln!("  └───────────────────────────────────┴───────────────────────────────────────");
        }
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
    let process_start = std::time::Instant::now();
    let paragraphs = process_paragraphs(content_lines, &rules.paragraph);
    let process_duration = process_start.elapsed();
    
    // 生成章节slug
    let slug_start = std::time::Instant::now();
    let slug = slugify(raw_title);
    let slug_duration = slug_start.elapsed();
    
    // 收集所有create_chapter的计时信息，用于后续汇总
    if let Ok(args) = crate::cli::Args::try_parse_from(std::env::args_os()) {
        if args.debug {
            // 使用thread_local存储计时信息
            thread_local! {
                static CREATE_CHAPTER_TIMES: std::cell::RefCell<(u128, u128)> = const { std::cell::RefCell::new((0, 0)) };
            }
            
            CREATE_CHAPTER_TIMES.with(|times| {
                let mut times = times.borrow_mut();
                times.0 += process_duration.as_nanos();
                times.1 += slug_duration.as_nanos();
            });
        }
    }
    
    Ok(Chapter {
        title: raw_title.trim().to_string(),
        paragraphs,
        slug,
        start_line,
        end_line,
    })
}

// 编译一次正则表达式，避免重复编译
lazy_static! {
    // 1. 修复 <ahref...> 这种标签名和属性之间缺少空格的情况
    static ref RE_TAG_SPACE: Regex = Regex::new(r#"<([a-zA-Z][a-zA-Z0-9]*)(href|src|target|alt|width|height|title|class|id|bl_id|a_id|b_id)"#).unwrap();
    // 2. 修复 hrefhttp:// 这种缺少等号和引号的情况
    static ref RE_HREF: Regex = Regex::new(r#"href(http[s]?://[^">]+)"#).unwrap();
    // 3. 修复 srchttp:// 这种缺少等号和引号的情况
    static ref RE_SRC: Regex = Regex::new(r#"src(http[s]?://[^">]+)"#).unwrap();
    // 4. 修复 target_blank 这种缺少等号和引号的情况，保留下划线
    static ref RE_ATTR_UNDERSCORE: Regex = Regex::new(r#"(target|alt|title|class|id|bl_id|a_id|b_id|width|height)_([^\s">]+)"#).unwrap();
    // 5. 修复 bl_id1234 这种缺少等号和引号的情况
    static ref RE_ATTR_NUM: Regex = Regex::new(r#"(bl_id|a_id|b_id|width|height)(\d+)"#).unwrap();
    // 6. 修复属性之间缺少空格的问题（如 target="_blank"href="..."）
    static ref RE_ATTR_NO_SPACE: Regex = Regex::new(r#""([a-zA-Z][a-zA-Z0-9_]*)="#).unwrap();
    // 7. 清理属性值前面的多余空格（如 href=" http://）
    static ref RE_ATTR_LEADING_SPACE: Regex = Regex::new(r#"="\s+"#).unwrap();
    // 8. 清理多余的空格
    static ref RE_EXTRA_SPACE: Regex = Regex::new(r#"\s+">"#).unwrap();
    // 9. 修复引号之间的多余空格
    static ref RE_QUOTE_SPACE: Regex = Regex::new(r#""\s+""#).unwrap();
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
            let mut escaped_line = String::with_capacity(processed_line.len() * 2); // 预分配空间减少扩容
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
            fixed_line = RE_TAG_SPACE.replace_all(&fixed_line, r#"<$1 $2"#).to_string();
            
            // 2. 修复 hrefhttp:// 这种缺少等号和引号的情况
            fixed_line = RE_HREF.replace_all(&fixed_line, r#"href="$1""#).to_string();
            
            // 3. 修复 srchttp:// 这种缺少等号和引号的情况
            fixed_line = RE_SRC.replace_all(&fixed_line, r#"src="$1""#).to_string();
            
            // 4. 修复 target_blank 这种缺少等号和引号的情况，保留下划线
            fixed_line = RE_ATTR_UNDERSCORE.replace_all(&fixed_line, r#"$1="$2""#).to_string();
            
            // 5. 修复 bl_id1234 这种缺少等号和引号的情况
            fixed_line = RE_ATTR_NUM.replace_all(&fixed_line, r#"$1="$2""#).to_string();
            
            // 11. 修复属性之间缺少空格的问题（如 target="_blank"href="..."）
            fixed_line = RE_ATTR_NO_SPACE.replace_all(&fixed_line, r#"" $1="#).to_string();
            
            // 12. 清理属性值前面的多余空格（如 href=" http://）
            fixed_line = RE_ATTR_LEADING_SPACE.replace_all(&fixed_line, r#"="#).to_string();
            
            // 13. 清理多余的空格
            fixed_line = RE_EXTRA_SPACE.replace_all(&fixed_line, r#">"#).to_string();
            
            // 14. 修复引号之间的多余空格
            fixed_line = RE_QUOTE_SPACE.replace_all(&fixed_line, r#"""#).to_string();
            
            current_paragraph.push_str(&fixed_line);
        }
    }
    
    // 添加最后一个段落（如果非空）
    if !current_paragraph.is_empty() {
        paragraphs.push(current_paragraph.trim().to_string());
    }
    
    paragraphs
}
