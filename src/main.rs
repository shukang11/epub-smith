use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use log::info;
use std::time::Duration;

struct SnapshotLoadArgs {
    file: std::path::PathBuf,
    output: std::path::PathBuf,
    rules: Option<std::path::PathBuf>,
    author: Option<String>,
    title: Option<String>,
    language: String,
    check: bool,
    style: Option<std::path::PathBuf>,
    cover: Option<String>,
    debug: bool,
}

// Initialize internationalization support
rust_i18n::i18n!("locales", fallback = "en");

// Import t macro for translation
use rust_i18n::t;

use epub_smith::output::GLOBAL_OUTPUT;
use epub_smith::utils::coherence::check_chapter_coherence;
use epub_smith::{
    cli::{
        Args, Commands, ConvertArgs, DoctorArgs, PreviewCommands, SnapshotCommands,
        TemplateCommands,
    },
    config::Config,
    doctor::{analyze_book, build_recommended_commands},
    export::export_template,
    packager::package_epub,
    parser::parse_txt,
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
                && output.status.success()
            {
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

    match args.command {
        Commands::Convert(convert_args) => {
            handle_convert_command(convert_args, args.debug)?;
        }
        Commands::Template(TemplateCommands::Export { directory }) => {
            export_template(&directory)?;
        }
        Commands::Snapshot(SnapshotCommands::Save(snapshot_args)) => {
            handle_snapshot_save_command(
                snapshot_args.input,
                snapshot_args.rules,
                snapshot_args.output,
                args.debug,
            )?;
        }
        Commands::Snapshot(SnapshotCommands::Load(snapshot_args)) => {
            let args = SnapshotLoadArgs {
                file: snapshot_args.file,
                output: snapshot_args.output,
                rules: snapshot_args.rules,
                author: snapshot_args.author,
                title: snapshot_args.title,
                language: snapshot_args.language,
                check: snapshot_args.check,
                style: snapshot_args.style,
                cover: snapshot_args.cover,
                debug: args.debug,
            };
            handle_snapshot_load_command(args)?;
        }
        Commands::Preview(PreviewCommands::Outline(preview_args)) => {
            handle_preview_outline_command(preview_args.input, preview_args.rules, args.debug)?;
        }
        Commands::Preview(PreviewCommands::DryRun(preview_args)) => {
            handle_preview_dryrun_command(preview_args.input, preview_args.rules, args.debug)?;
        }
        Commands::Doctor(doctor_args) => {
            handle_doctor_command(doctor_args, args.debug)?;
        }
    }

    Ok(())
}

/// 处理 doctor 命令
fn handle_doctor_command(args: DoctorArgs, debug: bool) -> Result<()> {
    let use_stdin = !args.input.is_empty() && args.input[0].to_string_lossy() == "-";
    if args.input.is_empty() && !use_stdin {
        anyhow::bail!("No input files provided");
    }

    let total_start = std::time::Instant::now();
    let config = Config::from_doctor_args(args.rules.clone(), args.encoding.clone())?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("诊断中：正在解析文本与章节...");
    spinner.enable_steady_tick(Duration::from_millis(120));

    let parse_start = std::time::Instant::now();
    let mut book =
        parse_txt(&args.input, &config).with_context(|| "Failed to parse input files")?;
    let parse_duration = parse_start.elapsed();

    spinner.finish_with_message("诊断解析完成");
    book.meta = config.merge_preview_meta(book.meta.title.as_str());

    let report = analyze_book(&book, &config.rules, args.max_samples);

    GLOBAL_OUTPUT.title("Doctor 诊断报告");
    GLOBAL_OUTPUT.empty_line();
    GLOBAL_OUTPUT.info(format!(
        "输入文件：{}",
        args.input
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    ));
    GLOBAL_OUTPUT.info(format!("章节总数：{}", report.total_chapters));
    GLOBAL_OUTPUT.info(format!(
        "可提取章节号：{}，无章节号：{}",
        report.numbered_chapters, report.unnumbered_chapters
    ));
    GLOBAL_OUTPUT.info(format!(
        "重复章节号：{}，顺序异常：{}",
        report.duplicate_count, report.order_issue_count
    ));
    GLOBAL_OUTPUT.empty_line();

    GLOBAL_OUTPUT.bold("分类结果");
    if report.has_data_anomalies() {
        GLOBAL_OUTPUT.warning("1) 源文本数据异常：存在重复或倒序章节号");
    } else {
        GLOBAL_OUTPUT.success("1) 源文本数据异常：未发现明显重复/倒序章节号");
    }

    if report.has_rule_or_parsing_risk() {
        GLOBAL_OUTPUT.warning("2) 规则/解析风险：存在潜在误判或漏判风险");
    } else {
        GLOBAL_OUTPUT.success("2) 规则/解析风险：未发现明显风险");
    }
    GLOBAL_OUTPUT.empty_line();

    if report.rule_risks.contains_zhang_unit {
        GLOBAL_OUTPUT.warning("规则风险：当前规则包含“张”单位，容易把正文误判成章节标题");
    }
    if report.rule_risks.missing_large_chinese_number_support {
        GLOBAL_OUTPUT.warning("规则风险：中文数字模式可能缺少“百/千/万”，可能导致大章节号漏识别");
    }
    if !report.suspicious_title_samples.is_empty() {
        GLOBAL_OUTPUT.warning(format!(
            "解析风险：检测到 {} 条疑似误判标题样本",
            report.suspicious_title_samples.len()
        ));
    }
    if !report.long_chapter_samples.is_empty() {
        GLOBAL_OUTPUT.warning(format!(
            "解析风险：检测到 {} 条超长章节样本（可能存在漏分）",
            report.long_chapter_samples.len()
        ));
    }
    if report.rule_risks.contains_zhang_unit
        || report.rule_risks.missing_large_chinese_number_support
        || !report.suspicious_title_samples.is_empty()
        || !report.long_chapter_samples.is_empty()
    {
        GLOBAL_OUTPUT.empty_line();
    }

    if !report.duplicate_samples.is_empty() {
        GLOBAL_OUTPUT.bold("重复章节号样本");
        for issue in &report.duplicate_samples {
            GLOBAL_OUTPUT.info(format!(
                "- 第{}：\"{}\" ({}) <-> \"{}\" ({})",
                issue.number,
                issue.first.title,
                issue.first.line_range(),
                issue.second.title,
                issue.second.line_range()
            ));
        }
        GLOBAL_OUTPUT.empty_line();
    }

    if !report.order_samples.is_empty() {
        GLOBAL_OUTPUT.bold("章节号顺序异常样本");
        for issue in &report.order_samples {
            GLOBAL_OUTPUT.info(format!(
                "- 第{} -> 第{}：\"{}\" ({}) -> \"{}\" ({})",
                issue.previous_number,
                issue.current_number,
                issue.previous.title,
                issue.previous.line_range(),
                issue.current.title,
                issue.current.line_range()
            ));
        }
        GLOBAL_OUTPUT.empty_line();
    }

    if !report.suspicious_title_samples.is_empty() {
        GLOBAL_OUTPUT.bold("疑似误判标题样本");
        for sample in &report.suspicious_title_samples {
            GLOBAL_OUTPUT.info(format!("- \"{}\" ({})", sample.title, sample.line_range()));
        }
        GLOBAL_OUTPUT.empty_line();
    }

    if !report.long_chapter_samples.is_empty() {
        GLOBAL_OUTPUT.bold("超长章节样本");
        for sample in &report.long_chapter_samples {
            GLOBAL_OUTPUT.info(format!(
                "- \"{}\" ({}，约 {} 行)",
                sample.chapter.title,
                sample.chapter.line_range(),
                sample.line_count
            ));
        }
        GLOBAL_OUTPUT.empty_line();
    }

    GLOBAL_OUTPUT.bold("建议下一步命令");
    for command in build_recommended_commands(&args.input, args.rules.as_deref(), &report) {
        GLOBAL_OUTPUT.info(format!("- {command}"));
    }
    GLOBAL_OUTPUT.empty_line();

    if debug {
        let metrics = [
            ("Text parsing", parse_duration),
            ("Total time", total_start.elapsed()),
        ];
        GLOBAL_OUTPUT.performance_summary("=== Performance Summary ===", &metrics);
    }

    Ok(())
}

/// 处理 convert 命令
fn handle_convert_command(args: ConvertArgs, debug: bool) -> Result<()> {
    info!("Starting EpubSmith v{}", env!("CARGO_PKG_VERSION"));
    info!("Input files: {:?}", args.input);

    // 检查是否使用了标准输入
    let use_stdin = !args.input.is_empty() && args.input[0].to_string_lossy() == "-";

    // 确保有输入文件
    if args.input.is_empty() && !use_stdin {
        anyhow::bail!("No input files provided");
    }

    // 开始总计时
    let total_start = std::time::Instant::now();

    let config = Config::from_convert_args(&args)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-parsing-text"));
    spinner.enable_steady_tick(Duration::from_millis(120));

    // 开始解析计时
    let parse_start = std::time::Instant::now();
    let mut book =
        parse_txt(&args.input, &config).with_context(|| "Failed to parse input files")?;
    let _parse_duration = parse_start.elapsed();

    spinner.finish_with_message(t!("success-text-parsed"));

    book.meta = config.merge_meta(&args, book.meta.title.as_str());

    check_chapter_coherence(&book.chapters, &config.rules.chapter_number_extraction);

    // 继续渲染和打包EPUB
    goto_render_and_package(book, config, total_start, debug)?;

    Ok(())
}

/// 处理 snapshot save 命令
fn handle_snapshot_save_command(
    input: Vec<std::path::PathBuf>,
    rules: Option<std::path::PathBuf>,
    output: std::path::PathBuf,
    debug: bool,
) -> Result<()> {
    // 检查是否使用了标准输入
    let use_stdin = !input.is_empty() && input[0].to_string_lossy() == "-";

    // 确保有输入文件
    if input.is_empty() && !use_stdin {
        anyhow::bail!("No input files provided");
    }

    // 开始总计时
    let total_start = std::time::Instant::now();

    let config = Config::from_snapshot_save_args(&input, rules)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-parsing-text"));
    spinner.enable_steady_tick(Duration::from_millis(120));

    // 开始解析计时
    let parse_start = std::time::Instant::now();
    let mut book = parse_txt(&input, &config).with_context(|| "Failed to parse input files")?;
    let parse_duration = parse_start.elapsed();

    spinner.finish_with_message(t!("success-text-parsed"));

    book.meta = config.merge_snapshot_meta(book.meta.title.as_str());

    check_chapter_coherence(&book.chapters, &config.rules.chapter_number_extraction);

    // 将book结构序列化为JSON并写入文件
    let snapshot_content = serde_json::to_string_pretty(&book)?;
    std::fs::write(&output, snapshot_content)
        .with_context(|| format!("Failed to write snapshot to: {}", output.display()))?;
    GLOBAL_OUTPUT.success(format!("Snapshot exported to: {}", output.display()));

    // 打印计时信息
    if debug {
        let metrics = [
            ("Text parsing", parse_duration),
            ("Total time", total_start.elapsed()),
        ];
        GLOBAL_OUTPUT.performance_summary("=== Performance Summary ===", &metrics);
    }

    Ok(())
}

/// 处理 snapshot load 命令
fn handle_snapshot_load_command(args: SnapshotLoadArgs) -> Result<()> {
    // 开始总计时
    let total_start = std::time::Instant::now();
    let config = Config::from_snapshot_load_args(
        args.output,
        args.rules,
        args.check,
        args.style,
        args.cover,
    )?;

    // 读取并解析快照文件
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Reading snapshot file...");
    spinner.enable_steady_tick(Duration::from_millis(120));

    let snapshot_content = std::fs::read_to_string(&args.file)
        .with_context(|| format!("Failed to read snapshot from: {}", args.file.display()))?;
    let mut book: epub_smith::models::Book = serde_json::from_str(&snapshot_content)
        .with_context(|| format!("Failed to parse snapshot from: {}", args.file.display()))?;

    spinner.finish_with_message("Snapshot loaded successfully");

    // 更新元数据
    book.meta = config.merge_snapshot_load_meta(
        args.title.as_deref(),
        args.author.as_deref(),
        args.language.as_str(),
        book.meta.title.as_str(),
    );

    // 跳过快照导入时的章节连贯性检查，因为快照是手动调整过的
    info!("Skipping chapter coherence check for snapshot input");

    // 继续渲染和打包EPUB
    goto_render_and_package(book, config, total_start, args.debug)?;
    Ok(())
}

/// 处理 preview outline 命令
fn handle_preview_outline_command(
    input: Vec<std::path::PathBuf>,
    rules: Option<std::path::PathBuf>,
    debug: bool,
) -> Result<()> {
    // 检查是否使用了标准输入
    let use_stdin = !input.is_empty() && input[0].to_string_lossy() == "-";

    // 确保有输入文件
    if input.is_empty() && !use_stdin {
        anyhow::bail!("No input files provided");
    }

    // 开始总计时
    let total_start = std::time::Instant::now();

    let config = Config::from_preview_args(rules)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-parsing-text"));
    spinner.enable_steady_tick(Duration::from_millis(120));

    // 开始解析计时
    let parse_start = std::time::Instant::now();
    let mut book = parse_txt(&input, &config).with_context(|| "Failed to parse input files")?;
    let parse_duration = parse_start.elapsed();

    spinner.finish_with_message(t!("success-text-parsed"));

    book.meta = config.merge_preview_meta(book.meta.title.as_str());

    check_chapter_coherence(&book.chapters, &config.rules.chapter_number_extraction);

    // 使用输出工具打印章节大纲
    GLOBAL_OUTPUT.title(t!("title-chapter-outline"));
    GLOBAL_OUTPUT.empty_line();

    for (i, chapter) in book.chapters.iter().enumerate() {
        let formatted = format!("{}. {}", i + 1, chapter.title);
        GLOBAL_OUTPUT.info(formatted);
    }
    GLOBAL_OUTPUT.empty_line();

    // 打印计时信息
    if debug {
        let metrics = [
            ("Text parsing", parse_duration),
            ("Total time", total_start.elapsed()),
        ];
        GLOBAL_OUTPUT.performance_summary("=== Performance Summary ===", &metrics);
    }

    Ok(())
}

/// 处理 preview dryrun 命令
fn handle_preview_dryrun_command(
    input: Vec<std::path::PathBuf>,
    rules: Option<std::path::PathBuf>,
    debug: bool,
) -> Result<()> {
    // 检查是否使用了标准输入
    let use_stdin = !input.is_empty() && input[0].to_string_lossy() == "-";

    // 确保有输入文件
    if input.is_empty() && !use_stdin {
        anyhow::bail!("No input files provided");
    }

    // 开始总计时
    let total_start = std::time::Instant::now();

    let config = Config::from_preview_args(rules)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-parsing-text"));
    spinner.enable_steady_tick(Duration::from_millis(120));

    // 开始解析计时
    let parse_start = std::time::Instant::now();
    let mut book = parse_txt(&input, &config).with_context(|| "Failed to parse input files")?;
    let parse_duration = parse_start.elapsed();

    spinner.finish_with_message(t!("success-text-parsed"));

    book.meta = config.merge_preview_meta(book.meta.title.as_str());

    check_chapter_coherence(&book.chapters, &config.rules.chapter_number_extraction);

    // 使用输出工具打印章节列表
    GLOBAL_OUTPUT.title(t!("title-detected-chapters"));
    GLOBAL_OUTPUT.empty_line();

    for (i, chapter) in book.chapters.iter().enumerate() {
        let line_range = format!("lines {}-{}", chapter.start_line + 1, chapter.end_line + 1);
        let formatted = format!("[{:03}] {} ({})\n", i + 1, chapter.title, line_range);
        GLOBAL_OUTPUT.info(formatted);
    }

    // 打印计时信息
    if debug {
        let metrics = [
            ("Text parsing", parse_duration),
            ("Total time", total_start.elapsed()),
        ];
        GLOBAL_OUTPUT.performance_summary("=== Performance Summary ===", &metrics);
    }

    Ok(())
}

/// 渲染和打包EPUB的通用函数
fn goto_render_and_package(
    book: epub_smith::models::Book,
    config: epub_smith::config::Config,
    total_start: std::time::Instant,
    debug: bool,
) -> Result<()> {
    // 开始渲染计时
    let render_start = std::time::Instant::now();
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(t!("processing-rendering-xhtml"));
    spinner.enable_steady_tick(Duration::from_millis(120));

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
    if debug {
        let metrics = [
            ("XHTML rendering", render_duration),
            ("EPUB packaging", package_duration),
            ("Total time", total_start.elapsed()),
        ];
        GLOBAL_OUTPUT.performance_summary("=== Performance Summary ===", &metrics);
    }

    Ok(())
}
