use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 生成书籍标识符
fn generate_identifier() -> String {
    format!("urn:uuid:{}", uuid::Uuid::new_v4())
}

/// 生成最后修改时间
fn generate_modified() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// 书籍元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// 书籍标题
    pub title: String,
    /// 书籍作者
    pub author: String,
    /// 书籍语言
    pub language: String,
    /// 书籍标识符（反序列化时可选，会自动生成）
    #[serde(default = "generate_identifier")]
    pub identifier: String,
    /// 最后修改时间（反序列化时可选，会自动生成）
    #[serde(default = "generate_modified")]
    pub modified: String,
    /// 封面图片路径
    pub cover: Option<PathBuf>,
}

/// 章节结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// 章节标题
    pub title: String,
    /// 章节段落列表
    pub paragraphs: Vec<String>,
    /// 章节slug（用于文件名）
    pub slug: String,
    /// 章节起始行号
    pub start_line: usize,
    /// 章节结束行号
    pub end_line: usize,
}

/// 书籍结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    /// 书籍元数据
    pub meta: Meta,
    /// 书籍章节列表
    pub chapters: Vec<Chapter>,
}

/// 段落处理规则
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParagraphRules {
    /// 是否合并连续非空行
    pub merge_lines: bool,
    /// 是否修剪行首行尾空白
    pub trim_whitespace: bool,
}

/// 章节解析规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterRules {
    /// 章节检测正则表达式列表
    pub regex: Vec<String>,
}

/// 单个章节序号提取规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterNumberRule {
    /// 正则表达式模式
    pub pattern: String,
    /// 捕获组索引（从1开始）
    pub capture_group: usize,
    /// 数字类型：auto, arabic, chinese, english, roman
    pub number_type: String,
}

/// 章节序号提取规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterNumberExtraction {
    /// 规则是一个数组，按优先级匹配
    pub rules: Vec<ChapterNumberRule>,
}

/// 规则文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    /// 章节解析规则
    pub chapter: ChapterRules,
    /// 章节序号提取规则
    pub chapter_number_extraction: Option<ChapterNumberExtraction>,
    /// 段落处理规则
    pub paragraph: ParagraphRules,
    /// 元数据配置（可选）
    pub meta: Option<Meta>,
}

/// Meta的默认实现
impl Default for Meta {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            author: "Unknown".to_string(),
            language: "zh-CN".to_string(),
            identifier: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
            modified: chrono::Utc::now().to_rfc3339(),
            cover: None,
        }
    }
}

/// ChapterRules的默认实现
impl Default for ChapterRules {
    fn default() -> Self {
        Self {
            regex: vec![
                r"^(?:第)?[一二三四五六七八九十0-9]+章".to_string(),
                r"^(?:第)?[一二三四五六七八九十0-9]+节".to_string(),
                r"^(?:第)?[一二三四五六七八九十0-9]+卷".to_string(),
                r"^(?:第)?[一二三四五六七八九十0-9]+张".to_string(),
                r"^Chapter\s+".to_string(),
                r"^Section\s+".to_string(),
                r"^Part\s+".to_string(),
                r"^番外".to_string(),
                r"^序".to_string(),
                r"^尾声".to_string(),
                r"^后记".to_string(),
                r"^附录".to_string(),
            ],
        }
    }
}

/// ChapterNumberExtraction的默认实现
impl Default for ChapterNumberExtraction {
    fn default() -> Self {
        Self {
            rules: vec![
                // 中文格式：第X章, X章, 第一章
                ChapterNumberRule {
                    pattern: "^(?:第)?([一二三四五六七八九十0-9]+)[章节卷张节]".to_string(),
                    capture_group: 1,
                    number_type: "auto".to_string(),
                },
                // 英文格式：Chapter 1, Section 2
                ChapterNumberRule {
                    pattern: "^(?:Chapter|Section|Part)\\s+([a-zA-Z0-9]+)".to_string(),
                    capture_group: 1,
                    number_type: "auto".to_string(),
                },
                // 罗马数字格式：Chapter I, Section II
                ChapterNumberRule {
                    pattern: "^(?:Chapter|Section|Part)\\s+([IVXLCDMivxlcdm]+)".to_string(),
                    capture_group: 1,
                    number_type: "roman".to_string(),
                },
                // 纯数字格式：1, 2, 3
                ChapterNumberRule {
                    pattern: r"^(\d+)$".to_string(),
                    capture_group: 1,
                    number_type: "arabic".to_string(),
                },
            ],
        }
    }
}

/// Rules的默认实现
impl Default for Rules {
    fn default() -> Self {
        Self {
            chapter: ChapterRules::default(),
            chapter_number_extraction: None,
            paragraph: ParagraphRules::default(),
            meta: None,
        }
    }
}
