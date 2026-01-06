use anyhow::{Context, Result};
use clap::Parser;
use console::style;
use log::info;

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
                println!("{}", style(format!("⚠️  警告：发现重复的章节号: {}", num)).yellow());
            }
        }
        
        // 检查章节号是否递增
        let mut is_increasing = true;
        for i in 1..numbered_chapters.len() {
            if numbered_chapters[i] <= numbered_chapters[i-1] {
                is_increasing = false;
                println!("{}", style(format!("⚠️  警告：章节号不是递增的，从 {} 到 {}", numbered_chapters[i-1], numbered_chapters[i])).yellow());
            }
        }
        
        if is_increasing {
            println!("{}", style("✅ 章节号是递增的，符合要求").green());
        }
    } else {
        // 如果没有提取到章节号，不进行连贯性检查
        println!("{}", style("ℹ️  未检测到明确的章节号，跳过连贯性检查").cyan());
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

    info!("Starting BookSmith v{}", env!("CARGO_PKG_VERSION"));
    info!("Input: {}", args.input.display());

    // 加载配置
    let config = Config::from_args(&args)?;

    // 解析TXT文件
    let mut book = parse_txt(&args.input, &config)
        .with_context(|| format!("Failed to parse input file: {}", args.input.display()))?;

    // 合并元数据
    book.meta = config.merge_meta(&args, book.meta.title.as_str());

    // 检查章节连贯性
    check_chapter_coherence(&book.chapters, &config.rules.chapter_number_extraction);

    // 处理dry-run模式
    if args.dry_run {
        println!("{}", style("Detected chapters:").bold());
        for (i, chapter) in book.chapters.iter().enumerate() {
            println!("[{:03}] {} (lines {}-{})
", i + 1, chapter.title, chapter.start_line + 1, chapter.end_line + 1);
        }
        return Ok(());
    }

    // 处理print-outline模式
    if args.print_outline {
        println!("{}", style("Chapter outline:").bold());
        for (i, chapter) in book.chapters.iter().enumerate() {
            println!("{}. {}", i + 1, chapter.title);
        }
        return Ok(());
    }

    // 渲染XHTML
    let xhtml_files = render_book(&book, &config)
        .with_context(|| "Failed to render XHTML files")?;

    // 打包EPUB
    package_epub(&book, &xhtml_files, &config)
        .with_context(|| format!("Failed to package EPUB to: {}", config.output.display()))?;

    println!("{}", style("✓ EPUB generated successfully!").green().bold());
    println!("Output: {}", style(config.output.display()).blue());

    Ok(())
}
