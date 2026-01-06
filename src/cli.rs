use clap::Parser;
use std::path::PathBuf;

/// 默认输出文件名
pub const DEFAULT_OUTPUT_FILENAME: &str = "book.epub";

/// A predictable, explainable, and reusable TXT to EPUB CLI tool
#[derive(Parser, Debug)]
#[command(name = "booksmith")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Convert TXT files to EPUB with predictable results", long_about = None)]
pub struct Args {
    /// 输入TXT文件或目录
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// 规则文件路径
    #[arg(short = 'r', long = "rules", value_name = "RULES")]
    pub rules: Option<PathBuf>,

    /// 输出EPUB文件路径
    #[arg(short = 'o', long = "output", value_name = "OUTPUT", default_value = DEFAULT_OUTPUT_FILENAME)]
    pub output: PathBuf,

    /// 强制指定输入文件编码
    #[arg(short = 'e', long = "encoding", value_name = "ENCODING")]
    pub encoding: Option<String>,

    /// 仅显示章节结构，不生成EPUB
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// 输出章节大纲
    #[arg(long = "print-outline")]
    pub print_outline: bool,

    /// 解释解析过程
    #[arg(long = "explain")]
    pub explain: bool,

    /// 指定书名
    #[arg(long = "title", value_name = "TITLE")]
    pub title: Option<String>,

    /// 指定作者
    #[arg(long = "author", value_name = "AUTHOR")]
    pub author: Option<String>,

    /// 指定封面图片路径
    #[arg(long = "cover", value_name = "COVER")]
    pub cover: Option<PathBuf>,

    /// 指定书籍语言
    #[arg(long = "language", value_name = "LANGUAGE", default_value = "zh-CN")]
    pub language: String,

    /// 调用epubcheck校验EPUB
    #[arg(long = "check")]
    pub check: bool,

    /// 显示详细日志
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}
