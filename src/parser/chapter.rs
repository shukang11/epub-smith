use crate::output::GLOBAL_OUTPUT;
use crate::utils::html;
use anyhow::{Context, Result};
use clap::Parser;
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
    let regex_patterns: Vec<Regex> = rules
        .chapter
        .regex
        .iter()
        .map(|pattern| {
            Regex::new(pattern).with_context(|| format!("Invalid regex pattern: {pattern}"))
        })
        .collect::<Result<_>>()?;
    let regex_duration = regex_start.elapsed();

    // 查找所有章节起始位置
    let mut chapter_starts: Vec<(usize, &str)> = Vec::new();

    if explain {
        GLOBAL_OUTPUT.info(t!("explain-using-regex"));
        for (i, pattern) in rules.chapter.regex.iter().enumerate() {
            GLOBAL_OUTPUT.info(format!("  {}. {}", i + 1, pattern));
        }
        GLOBAL_OUTPUT.empty_line();

        GLOBAL_OUTPUT.info(t!("explain-detecting-chapters"));
    }

    let detect_start = std::time::Instant::now();
    for (line_num, line) in lines.iter().enumerate() {
        // 检查该行是否匹配任何章节模式
        for (i, regex) in regex_patterns.iter().enumerate() {
            if regex.is_match(line) {
                if explain {
                    GLOBAL_OUTPUT.info(t!(
                        "explain-matched-line",
                        line = line_num + 1,
                        index = i + 1,
                        content = line.trim()
                    ));
                }
                chapter_starts.push((line_num, line));
                break;
            }
        }
    }
    let detect_duration = detect_start.elapsed();

    // 如果没有找到章节，将整个文件视为一个章节
    if chapter_starts.is_empty() {
        let end_line = if lines.is_empty() { 0 } else { lines.len() - 1 };
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
    if let Ok(args) = crate::cli::Args::try_parse_from(std::env::args_os())
        && args.debug
    {
        eprintln!(
            "  ┌─────────────────────────────────────────────────────────────────────────────"
        );
        eprintln!("  │ Chapter Parsing Details");
        eprintln!("  ├───────────────────────────────────┬───────────────────────────────────────");
        eprintln!("  │ Step                              │ Duration                              ");
        eprintln!("  ├───────────────────────────────────┼───────────────────────────────────────");
        eprintln!("  │ Regex compilation                 │ {regex_duration:<37.2?}");
        eprintln!("  │ Chapter detection                 │ {detect_duration:<37.2?}");
        eprintln!("  │ Chapter creation                  │ {create_duration:<37.2?}");
        eprintln!("  └───────────────────────────────────┴───────────────────────────────────────");
    }

    Ok(chapters)
}

/// 从行范围创建章节
fn create_chapter(
    lines: &[&str],
    start_line: usize,
    end_line: usize,
    raw_title: &str,
    rules: &Rules,
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
    if let Ok(args) = crate::cli::Args::try_parse_from(std::env::args_os())
        && args.debug
    {
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
            let escaped_line = html::escape_html(processed_line);

            // 修复错误格式的HTML标签
            let fixed_line = html::fix_html_tags(&escaped_line);

            current_paragraph.push_str(&fixed_line);
        }
    }

    // 添加最后一个段落（如果非空）
    if !current_paragraph.is_empty() {
        paragraphs.push(current_paragraph.trim().to_string());
    }

    paragraphs
}
