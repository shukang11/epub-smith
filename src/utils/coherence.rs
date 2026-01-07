use crate::models::{Chapter, ChapterNumberExtraction};
use crate::output::GLOBAL_OUTPUT;
use crate::utils::number::extract_chapter_number;

// 初始化国际化支持
rust_i18n::i18n!("locales", fallback = "en");

// 导入t宏用于翻译
use rust_i18n::t;

/// 检查章节编号的一致性
pub fn check_chapter_coherence(
    chapters: &[Chapter],
    extraction_rules: &Option<ChapterNumberExtraction>,
) {
    let chapter_nums: Vec<Option<usize>> = chapters
        .iter()
        .map(|chapter| extract_chapter_number(&chapter.title, extraction_rules))
        .collect();

    let numbered_chapters: Vec<usize> = chapter_nums.iter().filter_map(|num| *num).collect();

    if !numbered_chapters.is_empty() {
        let mut unique_nums = std::collections::HashSet::new();
        for num in &numbered_chapters {
            if !unique_nums.insert(*num) {
                GLOBAL_OUTPUT.warning(t!("warning-duplicate-chapter-number", number = num));
            }
        }

        let mut is_increasing = true;
        for i in 1..numbered_chapters.len() {
            if numbered_chapters[i] <= numbered_chapters[i - 1] {
                is_increasing = false;
                GLOBAL_OUTPUT.warning(t!(
                    "warning-chapter-not-increasing",
                    previous = numbered_chapters[i - 1],
                    current = numbered_chapters[i]
                ));
            }
        }

        if is_increasing {
            GLOBAL_OUTPUT.success(t!("success-chapters-increasing"));
        }
    } else {
        GLOBAL_OUTPUT.info(t!("info-no-chapter-numbers"));
    }
}
