use clap::Parser;
use std::path::PathBuf;

/// 默认输出文件名
pub const DEFAULT_OUTPUT_FILENAME: &str = "book.epub";

/// A predictable, explainable, and reusable TXT to EPUB CLI tool
#[derive(Parser, Debug)]
#[command(
    name = "booksmith",
    author = "Your Name <your.email@example.com>",
    version = env!("CARGO_PKG_VERSION"),
    about = "Convert TXT files to EPUB with predictable results",
    long_about = "BookSmith is a CLI tool that converts plain text files to well-structured EPUB books with predictable results.",
    after_help = "For more information, see https://github.com/yourusername/booksmith\n\nExamples:\n  booksmith my_book.txt\n  booksmith --author \"John Doe\" --title \"My Book\" input.txt\n  booksmith --rules custom_rules.toml --output my_book.epub input.txt",
    arg_required_else_help = true
)]
pub struct Args {
    /// 输入TXT文件或目录
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// 规则文件路径
    #[arg(short = 'r', long = "rules", value_name = "RULES", group = "output_options")]
    pub rules: Option<PathBuf>,

    /// 输出EPUB文件路径
    #[arg(short = 'o', long = "output", value_name = "OUTPUT", default_value = DEFAULT_OUTPUT_FILENAME, group = "output_options")]
    pub output: PathBuf,

    /// 强制指定输入文件编码
    #[arg(short = 'e', long = "encoding", value_name = "ENCODING", group = "output_options")]
    pub encoding: Option<String>,

    /// 仅显示章节结构，不生成EPUB
    #[arg(long = "dry-run", group = "preview_options")]
    pub dry_run: bool,

    /// 输出章节大纲
    #[arg(long = "print-outline", group = "preview_options")]
    pub print_outline: bool,

    /// 解释解析过程
    #[arg(long = "explain", group = "debug_options")]
    pub explain: bool,

    /// 指定书名
    #[arg(long = "title", value_name = "TITLE", group = "metadata_options")]
    pub title: Option<String>,

    /// 指定作者
    #[arg(long = "author", value_name = "AUTHOR", group = "metadata_options")]
    pub author: Option<String>,

    /// 指定封面图片路径
    #[arg(long = "cover", value_name = "COVER", group = "metadata_options")]
    pub cover: Option<PathBuf>,

    /// 指定书籍语言
    #[arg(long = "language", value_name = "LANGUAGE", default_value = "zh-CN", group = "metadata_options")]
    pub language: String,

    /// 调用epubcheck校验EPUB
    #[arg(long = "check", group = "output_options")]
    pub check: bool,

    /// 显示详细日志
    #[arg(short = 'v', long = "verbose", group = "debug_options")]
    pub verbose: bool,

    /// 指定输出语言（例如：en, zh）
    #[arg(long = "lang", value_name = "LANGUAGE")]
    pub lang: Option<String>,
}
