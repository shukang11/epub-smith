use clap::Parser;
use std::path::PathBuf;

/// 默认输出文件名
pub const DEFAULT_OUTPUT_FILENAME: &str = "book.epub";

/// A predictable, explainable, and reusable TXT to EPUB CLI tool
#[derive(Parser, Debug)]
#[command(
    name = "epub-smith",
    author = "Your Name <your.email@example.com>",
    version = env!("CARGO_PKG_VERSION"),
    about = "Convert TXT files to EPUB with predictable results",
    long_about = "EpubSmith is a CLI tool that converts plain text files to well-structured EPUB books with predictable results.",
    after_help = "For more information, see https://github.com/yourusername/epub-smith\n\nExamples:\n  epub-smith convert my_book.txt\n  epub-smith convert file1.txt file2.txt file3.txt\n  epub-smith convert *.txt -o combined.epub\n  epub-smith convert --author \"John Doe\" --title \"My Book\" input.txt\n  cat novel.txt | epub-smith convert - -o novel.epub\n  epub-smith template export my_templates\n  epub-smith snapshot save input.txt -o book.json\n  epub-smith snapshot load book.json -o output.epub",
    arg_required_else_help = true
)]
pub struct Args {
    /// 显示详细日志
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// 启用性能分析，显示各阶段耗时
    #[arg(long = "debug")]
    pub debug: bool,

    /// 指定输出语言（例如：en, zh）
    #[arg(long = "lang", value_name = "LANGUAGE")]
    pub lang: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Convert TXT files to EPUB (default command)
    #[command(
        about = "Convert TXT files to well-structured EPUB books",
        after_help = "Examples:\n  epub-smith convert my_book.txt\n  epub-smith convert file1.txt file2.txt\n  epub-smith convert *.txt -o combined.epub\n  cat novel.txt | epub-smith convert - -o novel.epub"
    )]
    Convert(ConvertArgs),

    /// Template management
    #[command(subcommand)]
    Template(TemplateCommands),

    /// Snapshot management
    #[command(subcommand)]
    Snapshot(SnapshotCommands),

    /// Preview functionality
    #[command(subcommand)]
    Preview(PreviewCommands),
}

#[derive(clap::Subcommand, Debug)]
pub enum TemplateCommands {
    /// Export style template to a directory
    Export {
        /// Directory to export the template to
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,
    },
}

#[derive(clap::Subcommand, Debug)]
pub enum SnapshotCommands {
    /// Save chapter structure to a snapshot file
    Save(SnapshotSaveArgs),

    /// Load chapter structure from a snapshot file
    Load(SnapshotLoadArgs),
}

#[derive(clap::Subcommand, Debug)]
pub enum PreviewCommands {
    /// Print chapter outline
    Outline(PreviewArgs),

    /// Show detailed chapter structure without generating EPUB
    DryRun(PreviewArgs),
}

/// Arguments for the convert command
#[derive(Parser, Debug)]
pub struct ConvertArgs {
    /// 输入TXT文件或目录，使用 - 表示从标准输入读取
    #[arg(value_name = "INPUT")]
    pub input: Vec<PathBuf>,

    /// 规则文件路径
    #[arg(short = 'r', long = "rules", value_name = "RULES")]
    pub rules: Option<PathBuf>,

    /// 输出EPUB文件路径
    #[arg(short = 'o', long = "output", value_name = "OUTPUT", default_value = DEFAULT_OUTPUT_FILENAME)]
    pub output: PathBuf,

    /// 强制指定输入文件编码
    #[arg(short = 'e', long = "encoding", value_name = "ENCODING")]
    pub encoding: Option<String>,

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

    /// 指定自定义CSS样式文件
    #[arg(long = "style", value_name = "CSS_FILE")]
    pub style: Option<PathBuf>,
}

/// Arguments for the snapshot save command
#[derive(Parser, Debug)]
pub struct SnapshotSaveArgs {
    /// Input TXT file or directory
    #[arg(value_name = "INPUT")]
    pub input: Vec<PathBuf>,

    /// Rules file path (optional)
    #[arg(short = 'r', long = "rules", value_name = "RULES")]
    pub rules: Option<PathBuf>,

    /// File to save the snapshot to
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output: PathBuf,
}

/// Arguments for the snapshot load command
#[derive(Parser, Debug)]
pub struct SnapshotLoadArgs {
    /// Snapshot file to use
    #[arg(value_name = "FILE")]
    pub file: PathBuf,

    /// Output EPUB file path
    #[arg(short = 'o', long = "output", value_name = "OUTPUT", default_value = DEFAULT_OUTPUT_FILENAME)]
    pub output: PathBuf,

    /// Rules file path (optional)
    #[arg(short = 'r', long = "rules", value_name = "RULES")]
    pub rules: Option<PathBuf>,

    /// Specify book author
    #[arg(long = "author", value_name = "AUTHOR")]
    pub author: Option<String>,

    /// Specify book title
    #[arg(long = "title", value_name = "TITLE")]
    pub title: Option<String>,

    /// Specify book language
    #[arg(long = "language", value_name = "LANGUAGE", default_value = "zh-CN")]
    pub language: String,

    /// Call epubcheck to validate EPUB
    #[arg(long = "check")]
    pub check: bool,

    /// Specify custom CSS style file
    #[arg(long = "style", value_name = "CSS_FILE")]
    pub style: Option<PathBuf>,
}

/// Arguments for preview commands
#[derive(Parser, Debug)]
pub struct PreviewArgs {
    /// Input TXT file or directory
    #[arg(value_name = "INPUT")]
    pub input: Vec<PathBuf>,

    /// Rules file path (optional)
    #[arg(short = 'r', long = "rules", value_name = "RULES")]
    pub rules: Option<PathBuf>,
}
