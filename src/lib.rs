// 初始化国际化支持
rust_i18n::i18n!("locales", fallback = "en");

pub mod cli;
pub mod config;
pub mod doctor;
pub mod export;
pub mod models;
pub mod output;
pub mod packager;
pub mod parser;
pub mod renderer;
pub mod utils;

// 导出t宏供外部使用
pub use rust_i18n::t;

// 重新导出工具函数，保持API兼容性
pub use crate::utils::html::*;
pub use crate::utils::number::*;
