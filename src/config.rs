use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use toml::from_str;

use crate::{
    cli::{Args, DEFAULT_OUTPUT_FILENAME},
    models::{Meta, Rules},
};

/// 应用配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 解析和处理规则
    pub rules: Rules,
    /// 输出EPUB文件路径
    pub output: PathBuf,
    /// 是否检查EPUB
    pub check: bool,
    /// 是否解释解析过程
    pub explain: bool,
    /// 输入文件编码（如果指定）
    pub encoding: Option<String>,
    /// 自定义CSS样式文件路径
    pub style: Option<PathBuf>,
}

impl Config {
    /// 从命令行参数创建配置
    pub fn from_args(args: &Args) -> Result<Self> {
        // 加载规则文件或使用默认规则
        let rules = if let Some(rules_path) = &args.rules {
            Config::load_rules(rules_path)?
        } else {
            Rules::default()
        };

        // 如果输出路径是默认值，且有输入文件，使用第一个输入文件的名称来生成输出文件名（替换扩展名）
        let output = if !args.input.is_empty() {
            // 检查输出路径是否是默认值
            let output_str = args.output.to_str().unwrap_or_default();
            if output_str == DEFAULT_OUTPUT_FILENAME {
                // 获取第一个输入文件的文件名，替换扩展名为.epub
                let first_input = &args.input[0];
                if first_input.to_string_lossy() != "-" {
                    // 不是stdin输入
                    if let Some(input_filename) =
                        first_input.file_name().and_then(|os_str| os_str.to_str())
                    {
                        let output_filename =
                            if let Some((name, _)) = input_filename.rsplit_once('.') {
                                format!("{}.epub", name)
                            } else {
                                format!("{}.epub", input_filename)
                            };
                        PathBuf::from(output_filename)
                    } else {
                        args.output.clone()
                    }
                } else {
                    args.output.clone()
                }
            } else {
                args.output.clone()
            }
        } else {
            args.output.clone()
        };

        Ok(Self {
            rules,
            output,
            check: args.check,
            explain: args.explain,
            encoding: args.encoding.clone(),
            style: args.style.clone(),
        })
    }

    /// 从TOML文件加载规则
    fn load_rules(path: &PathBuf) -> Result<Rules> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read rules file: {}", path.display()))?;

        let rules: Rules = from_str(&content)
            .with_context(|| format!("Failed to parse rules file: {}", path.display()))?;

        Ok(rules)
    }

    /// 合并命令行参数和规则文件中的元数据
    pub fn merge_meta(&self, args: &Args, default_title: &str) -> Meta {
        // 从规则文件获取元数据，如果不存在则使用默认值
        let mut meta = self.rules.meta.clone().unwrap_or_default();

        // 使用命令行参数覆盖元数据
        if let Some(title) = &args.title {
            meta.title = title.clone();
        } else if meta.title.is_empty() || meta.title == "Untitled" {
            // 如果标题为空或为默认的"Untitled"，则使用传入的default_title
            meta.title = default_title.to_string();
        }

        if let Some(author) = &args.author {
            meta.author = author.clone();
        } else if meta.author.is_empty() {
            meta.author = "Unknown".to_string();
        }

        if meta.language.is_empty() {
            meta.language = args.language.clone();
        }

        if let Some(cover) = &args.cover {
            meta.cover = Some(cover.clone());
        }

        // 如果没有提供标识符，生成一个新的UUID
        if meta.identifier.is_empty() {
            meta.identifier = format!("urn:uuid:{}", uuid::Uuid::new_v4());
        }

        // 设置修改时间为当前时间
        meta.modified = chrono::Utc::now().to_rfc3339();

        meta
    }
}
