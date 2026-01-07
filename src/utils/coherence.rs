use crate::models::{Chapter, ChapterNumberExtraction};
use crate::output::GLOBAL_OUTPUT;
use crate::utils::number::extract_chapter_number;

rust_i18n::i18n!("locales", fallback = "en");
use rust_i18n::t;

pub fn check_chapter_coherence(
    chapters: &[Chapter],
    extraction_rules: &Option<ChapterNumberExtraction>,
) {
    let chapter_nums: Vec<Option<usize>> = chapters
        .iter()
        .map(|chapter| extract_chapter_number(&chapter.title, extraction_rules))
        .collect();

    let numbered_chapters: Vec<(usize, &Chapter)> = chapter_nums
        .iter()
        .enumerate()
        .filter_map(|(idx, num)| num.map(|n| (n, &chapters[idx])))
        .collect();

    if !numbered_chapters.is_empty() {
        let mut seen_nums: std::collections::HashMap<usize, &Chapter> = std::collections::HashMap::new();
        for (num, chapter) in &numbered_chapters {
            if let Some(prev_chapter) = seen_nums.get(num) {
                GLOBAL_OUTPUT.warning(format!(
                    "⚠️ 警告：章节号 \"{}\" 重复 - \"{}\" 与 \"{}\"",
                    num, prev_chapter.title, chapter.title
                ));
            }
            seen_nums.insert(*num, *chapter);
        }

        let mut is_increasing = true;
        for i in 1..numbered_chapters.len() {
            let (prev_num, prev_chapter) = numbered_chapters[i - 1];
            let (curr_num, curr_chapter) = numbered_chapters[i];
            if curr_num <= prev_num {
                is_increasing = false;
                GLOBAL_OUTPUT.warning(format!(
                    "⚠️ 警告：章节号顺序异常 - \"{}\" (第{}) 之后是 \"{}\" (第{})",
                    prev_chapter.title, prev_num, curr_chapter.title, curr_num
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
