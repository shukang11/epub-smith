use anyhow::{Context, Result};
use clap::Parser;
use console::style;
use indicatif::ProgressBar;
use log::info;
use std::time::Duration;

// Initialize internationalization support
rust_i18n::i18n!("locales", fallback = "en");

// Import t macro for translation
use rust_i18n::t;

use booksmith::{cli::Args, config::Config, parser::parse_txt, renderer::render_book, packager::package_epub, extract_chapter_number, export::export_template};

/// Check chapter coherence
fn check_chapter_coherence(chapters: &[booksmith::models::Chapter], extraction_rules: &Option<booksmith::models::ChapterNumberExtraction>) {
    let chapter_nums: Vec<Option<usize>> = chapters
        .iter()
        .map(|chapter| extract_chapter_number(&chapter.title, extraction_rules))
        .collect();
    
    let numbered_chapters: Vec<usize> = chapter_nums
        .iter()
        .filter_map(|num| *num)
        .collect();
    
    if !numbered_chapters.is_empty() {
        let mut unique_nums = std::collections::HashSet::new();
        for num in &numbered_chapters {
            if !unique_nums.insert(*num) {
                println!("{}", style(t!("warning-duplicate-chapter-number", number = num)).yellow());
            }
        }
        
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
        println!("{}", style(t!("info-no-chapter-numbers")).cyan());
    }
}

fn main() -> Result<()> {
    let log_level = if Args::try_parse_from(std::env::args_os())
        .map(|args| args.verbose)
        .unwrap_or(false) {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Warn
    };

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

    let args = Args::parse();

    if let Some(ref export_dir) = args.export_template {
        return export_template(export_dir);
    }

    let lang = args.lang.as_deref().unwrap_or_default();
    if !lang.is_empty() {
        rust_i18n::set_locale(lang);
    } else {
        let supported_locales = ["en", "zh", "zh-CN", "zh-TW", "ja", "ko", "fr", "de", "es", "it"];

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("defaults")
                .args(&["read", "-g", "AppleLanguages"])
                .output()
            {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
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

        #[cfg(target_os = "windows")]
        {
            use winreg::enums::*;
            use winreg::RegKey;

            if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER) {
                if let Ok(international) = hkcu.open_subkey("Control Panel\\International") {
                    if let Ok(locale_name) = international.get_value::<String, _>("Locale") {
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
    info!("Input: {}", args.input.as_ref().unwrap().display());

    let config = Config::from_args(&args)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-parsing-text"));
    spinner.enable_steady_tick(Duration::from_millis(120));
    
    let mut book = parse_txt(args.input.as_ref().unwrap(), &config)
        .with_context(|| format!("Failed to parse input file: {}", args.input.as_ref().unwrap().display()))?;
    
    spinner.finish_with_message(t!("success-text-parsed"));

    book.meta = config.merge_meta(&args, book.meta.title.as_str());

    check_chapter_coherence(&book.chapters, &config.rules.chapter_number_extraction);

    if args.dry_run {
        println!("{}", style(t!("title-detected-chapters")).bold());
        for (i, chapter) in book.chapters.iter().enumerate() {
            println!("[{:03}] {} (lines {}-{})
", i + 1, chapter.title, chapter.start_line + 1, chapter.end_line + 1);
        }
        return Ok(());
    }

    if args.print_outline {
        println!("{}", style(t!("title-chapter-outline")).bold());
        for (i, chapter) in book.chapters.iter().enumerate() {
            println!("{}. {}", i + 1, chapter.title);
        }
        return Ok(());
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-rendering-xhtml"));
    spinner.enable_steady_tick(Duration::from_millis(120));
    
    let xhtml_files = render_book(&book, &config)
        .with_context(|| "Failed to render XHTML files")?;
    
    spinner.finish_with_message(t!("success-xhtml-rendered"));

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