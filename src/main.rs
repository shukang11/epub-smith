use anyhow::{Context, Result};
use clap::Parser;
use console::style;
use indicatif::ProgressBar;
use log::info;
use std::time::Duration;

// 初始化国际化支持
rust_i18n::i18n!("locales", fallback = "en");

// 导入t宏用于翻译
use rust_i18n::t;

use booksmith::{cli::Args, config::Config, parser::parse_txt, renderer::render_book, packager::package_epub, extract_chapter_number};

/// 检查章节的连贯性
fn check_chapter_coherence(chapters: &[booksmith::models::Chapter], extraction_rules: &Option<booksmith::models::ChapterNumberExtraction>) {
    // 提取章节号
    let chapter_nums: Vec<Option<usize>> = chapters
        .iter()
        .map(|chapter| extract_chapter_number(&chapter.title, extraction_rules))
        .collect();
    
    // 只保留有明确章节号的章节，忽略特殊章节（如番外、序、尾声等）
    let numbered_chapters: Vec<usize> = chapter_nums
        .iter()
        .filter_map(|num| *num)
        .collect();
    
    if !numbered_chapters.is_empty() {
        // 检查是否有重复的章节号
        let mut unique_nums = std::collections::HashSet::new();
        for num in &numbered_chapters {
            if !unique_nums.insert(*num) {
                println!("{}", style(t!("warning-duplicate-chapter-number", number = num)).yellow());
            }
        }
        
        // 检查章节号是否递增
        let mut is_increasing = true;
        for i in 1..numbered_chapters.len() {
            if numbered_chapters[i] <= numbered_chapters[i-1] {
                is_increasing = false;
                println!("{}", style(t!("warning-chapter-not-increasing", previous = numbered_chapters[i-1], current = numbered_chapters[i])).yellow());
            }
        }
        
        if is_increasing {
            println!("{}", style(t!("success-chapters-increasing")).green());
        }
    } else {
        // 如果没有提取到章节号，不进行连贯性检查
        println!("{}", style(t!("info-no-chapter-numbers")).cyan());
    }
}

fn main() -> Result<()> {
    // 根据是否为verbose模式，选择不同的日志级别
    let log_level = if Args::try_parse_from(std::env::args_os())
        .map(|args| args.verbose)
        .unwrap_or(false) {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Warn
    };

    // 初始化日志，只初始化一次
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()?;

    // 解析命令行参数
    let args = Args::parse();

    // 设置语言
    let lang = args.lang.as_deref().unwrap_or_default();
    if !lang.is_empty() {
        // 从命令行参数获取语言
        rust_i18n::set_locale(lang);
    } else {
        // 支持的语言列表
        let supported_locales = ["en", "zh", "zh-CN", "zh-TW", "ja", "ko", "fr", "de", "es", "it"];

        // 1. macOS 系统语言检测
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            // 读取 macOS 系统语言设置
            if let Ok(output) = Command::new("defaults")
                .args(&["read", "-g", "AppleLanguages"])
                .output()
            {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    // AppleLanguages 输出格式类似: ( "en-CN", "zh-CN" )
                    if let Some(lang_code) = stdout
                        .split(|c| c == '"' || c == '(' || c == ')' || c == ',')
                        .filter(|s| !s.trim().is_empty())
                        .next()
                    {
                        let lang_code = lang_code.split('-').next().unwrap_or(lang_code);
                        let lang_code = lang_code.split('_').next().unwrap_or(lang_code);

                        if supported_locales.contains(&lang_code) {
                            rust_i18n::set_locale(lang_code);
                        } else if lang_code.starts_with("zh") {
                            rust_i18n::set_locale("zh-CN");
                        } else if lang_code.starts_with("ja") {
                            rust_i18n::set_locale("ja");
                        } else if lang_code.starts_with("ko") {
                            rust_i18n::set_locale("ko");
                        }
                    }
                }
            }
        }

        // 2. Windows 系统语言检测
        #[cfg(target_os = "windows")]
        {
            use winreg::enums::*;
            use winreg::RegKey;

            if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER) {
                if let Ok(international) = hkcu.open_subkey("Control Panel\\International") {
                    if let Ok(locale_name) = international.get_value::<String, _>("Locale") {
                        // Windows locale name 示例: "zh-CN", "en-US"
                        let lang_code = if locale_name.len() >= 5 {
                            &locale_name[..5]
                        } else {
                            &locale_name
                        };

                        if supported_locales.contains(&lang_code) {
                            rust_i18n::set_locale(lang_code);
                        } else if lang_code.starts_with("zh") {
                            rust_i18n::set_locale("zh-CN");
                        } else if lang_code.starts_with("ja") {
                            rust_i18n::set_locale("ja");
                        } else if lang_code.starts_with("ko") {
                            rust_i18n::set_locale("ko");
                        }
                    }
                }
            }
        }

        // 3. Linux 系统语言检测
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/etc/default/locale") {
                for line in content.lines() {
                    if line.starts_with("LANG=") {
                        let lang_value = line.trim_start_matches("LANG=").trim_matches('"');
                        let lang_code = lang_value.split('.').next().unwrap_or(lang_value);
                        let lang_code = lang_code.split('_').next().unwrap_or(lang_code);

                        if supported_locales.contains(&lang_code) {
                            rust_i18n::set_locale(lang_code);
                        } else if lang_code.starts_with("zh") {
                            rust_i18n::set_locale("zh-CN");
                        } else if lang_code.starts_with("ja") {
                            rust_i18n::set_locale("ja");
                        } else if lang_code.starts_with("ko") {
                            rust_i18n::set_locale("ko");
                        }
                        break;
                    }
                }
            }
        }
    }

    info!("Starting BookSmith v{}", env!("CARGO_PKG_VERSION"));
    info!("Input: {}", args.input.display());

    // 加载配置
    let config = Config::from_args(&args)?;

    // 解析TXT文件
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-parsing-text"));
    spinner.enable_steady_tick(Duration::from_millis(120));
    
    let mut book = parse_txt(&args.input, &config)
        .with_context(|| format!("Failed to parse input file: {}", args.input.display()))?;
    
    spinner.finish_with_message(t!("success-text-parsed"));

    // 合并元数据
    book.meta = config.merge_meta(&args, book.meta.title.as_str());

    // 检查章节连贯性
    check_chapter_coherence(&book.chapters, &config.rules.chapter_number_extraction);

    // 处理dry-run模式
    if args.dry_run {
        println!("{}", style(t!("title-detected-chapters")).bold());
        for (i, chapter) in book.chapters.iter().enumerate() {
            println!("[{:03}] {} (lines {}-{})
", i + 1, chapter.title, chapter.start_line + 1, chapter.end_line + 1);
        }
        return Ok(());
    }

    // 处理print-outline模式
    if args.print_outline {
        println!("{}", style(t!("title-chapter-outline")).bold());
        for (i, chapter) in book.chapters.iter().enumerate() {
            println!("{}. {}", i + 1, chapter.title);
        }
        return Ok(());
    }

    // 渲染XHTML
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-rendering-xhtml"));
    spinner.enable_steady_tick(Duration::from_millis(120));
    
    let xhtml_files = render_book(&book, &config)
        .with_context(|| "Failed to render XHTML files")?;
    
    spinner.finish_with_message(t!("success-xhtml-rendered"));

    // 打包EPUB
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-packaging-epub"));
    spinner.enable_steady_tick(Duration::from_millis(120));
    
    package_epub(&book, &xhtml_files, &config)
        .with_context(|| format!("Failed to package EPUB to: {}", config.output.display()))?;
    
    spinner.finish_with_message(t!("success-epub-packaged"));

    println!("{}", style(t!("success-epub-generated")).green().bold());
    println!("{} {}", t!("label-output"), style(config.output.display()).blue());
    println!("{}", style(t!("info-total-chapters-processed", count = book.chapters.len())).cyan());

    Ok(())
}
