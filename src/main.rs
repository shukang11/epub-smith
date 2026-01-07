use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use log::info;
use std::time::Duration;

// Initialize internationalization support
rust_i18n::i18n!("locales", fallback = "en");

// Import t macro for translation
use rust_i18n::t;

use epub_smith::output::GLOBAL_OUTPUT;
use epub_smith::utils::coherence::check_chapter_coherence;
use epub_smith::{
    cli::Args, config::Config, export::export_template, packager::package_epub, parser::parse_txt,
    renderer::render_book,
};

fn main() -> Result<()> {
    let log_level = if Args::try_parse_from(std::env::args_os())
        .map(|args| args.verbose)
        .unwrap_or(false)
    {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Warn
    };

    fern::Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("[{}] {}", record.level(), message)))
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
        let supported_locales = [
            "en", "zh", "zh-CN", "zh-TW", "ja", "ko", "fr", "de", "es", "it",
        ];

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("defaults")
                .args(["read", "-g", "AppleLanguages"])
                .output()
                && output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if let Some(lang_code) = stdout
                        .split(&['"', '(', ')', ','])
                        .find(|s| !s.trim().is_empty())
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

        #[cfg(target_os = "windows")]
        {
            use winreg::RegKey;
            use winreg::enums::*;

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
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

    info!("Starting EpubSmith v{}", env!("CARGO_PKG_VERSION"));
    info!("Input files: {:?}", args.input);

    // 开始总计时
    let total_start = std::time::Instant::now();

    let config = Config::from_args(&args)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-parsing-text"));
    spinner.enable_steady_tick(Duration::from_millis(120));

    // 开始解析计时
    let parse_start = std::time::Instant::now();
    let mut book =
        parse_txt(&args.input, &config).with_context(|| "Failed to parse input files")?;
    let parse_duration = parse_start.elapsed();

    spinner.finish_with_message(t!("success-text-parsed"));

    book.meta = config.merge_meta(&args, book.meta.title.as_str());

    check_chapter_coherence(&book.chapters, &config.rules.chapter_number_extraction);

    if args.dry_run {
        // 使用输出工具打印章节列表
        GLOBAL_OUTPUT.title(t!("title-detected-chapters"));
        GLOBAL_OUTPUT.empty_line();

        for (i, chapter) in book.chapters.iter().enumerate() {
            let line_range = format!("lines {}-{}", chapter.start_line + 1, chapter.end_line + 1);
            let formatted = format!(
                "[{:03}] {} ({})
",
                i + 1,
                chapter.title,
                line_range
            );
            GLOBAL_OUTPUT.info(formatted);
        }

        // 打印计时信息
        if args.debug {
            let metrics = [
                ("Text parsing", parse_duration),
                ("Total time", total_start.elapsed()),
            ];
            GLOBAL_OUTPUT.performance_summary("=== Performance Summary ===", &metrics);
        }

        return Ok(());
    }

    if args.print_outline {
        // 使用输出工具打印章节大纲
        GLOBAL_OUTPUT.title(t!("title-chapter-outline"));
        GLOBAL_OUTPUT.empty_line();

        for (i, chapter) in book.chapters.iter().enumerate() {
            let formatted = format!("{}. {}", i + 1, chapter.title);
            GLOBAL_OUTPUT.info(formatted);
        }
        GLOBAL_OUTPUT.empty_line();

        // 打印计时信息
        if args.debug {
            let metrics = [
                ("Text parsing", parse_duration),
                ("Total time", total_start.elapsed()),
            ];
            GLOBAL_OUTPUT.performance_summary("=== Performance Summary ===", &metrics);
        }

        return Ok(());
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-rendering-xhtml"));
    spinner.enable_steady_tick(Duration::from_millis(120));

    // 开始渲染计时
    let render_start = std::time::Instant::now();
    let xhtml_files =
        render_book(&book, &config).with_context(|| "Failed to render XHTML files")?;
    let render_duration = render_start.elapsed();

    spinner.finish_with_message(t!("success-xhtml-rendered"));

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-packaging-epub"));
    spinner.enable_steady_tick(Duration::from_millis(120));

    // 开始打包计时
    let package_start = std::time::Instant::now();
    package_epub(&book, &xhtml_files, &config)
        .with_context(|| format!("Failed to package EPUB to: {}", config.output.display()))?;
    let package_duration = package_start.elapsed();

    spinner.finish_with_message(t!("success-epub-packaged"));

    // 打印成功消息
    GLOBAL_OUTPUT.success(t!("success-epub-generated"));
    GLOBAL_OUTPUT.info(format!(
        "{} {}",
        t!("label-output"),
        config.output.display()
    ));
    GLOBAL_OUTPUT.info(t!(
        "info-total-chapters-processed",
        count = book.chapters.len()
    ));
    GLOBAL_OUTPUT.empty_line();

    // 打印计时信息
    if args.debug {
        let metrics = [
            ("Text parsing", parse_duration),
            ("XHTML rendering", render_duration),
            ("EPUB packaging", package_duration),
            ("Total time", total_start.elapsed()),
        ];
        GLOBAL_OUTPUT.performance_summary("=== Performance Summary ===", &metrics);
    }

    Ok(())
}
