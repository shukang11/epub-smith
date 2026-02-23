use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::models::{Book, Chapter, ChapterNumberExtraction, Rules};
use crate::utils::number::extract_chapter_number;

const DEFAULT_MAX_SAMPLES: usize = 5;
const LONG_CHAPTER_LINE_THRESHOLD: usize = 2500;

#[derive(Debug, Clone)]
pub struct ChapterSample {
    pub title: String,
    pub start_line: usize,
    pub end_line: usize,
}

impl ChapterSample {
    fn from_chapter(chapter: &Chapter) -> Self {
        Self {
            title: chapter.title.clone(),
            start_line: chapter.start_line + 1,
            end_line: chapter.end_line + 1,
        }
    }

    pub fn line_range(&self) -> String {
        format!("lines {}-{}", self.start_line, self.end_line)
    }
}

#[derive(Debug, Clone)]
pub struct DuplicateIssue {
    pub number: usize,
    pub first: ChapterSample,
    pub second: ChapterSample,
}

#[derive(Debug, Clone)]
pub struct OrderIssue {
    pub previous_number: usize,
    pub previous: ChapterSample,
    pub current_number: usize,
    pub current: ChapterSample,
}

#[derive(Debug, Clone)]
pub struct LongChapterIssue {
    pub chapter: ChapterSample,
    pub line_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct RuleRiskFlags {
    pub contains_zhang_unit: bool,
    pub missing_large_chinese_number_support: bool,
}

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub total_chapters: usize,
    pub numbered_chapters: usize,
    pub unnumbered_chapters: usize,
    pub duplicate_count: usize,
    pub order_issue_count: usize,
    pub duplicate_samples: Vec<DuplicateIssue>,
    pub order_samples: Vec<OrderIssue>,
    pub suspicious_title_samples: Vec<ChapterSample>,
    pub long_chapter_samples: Vec<LongChapterIssue>,
    pub rule_risks: RuleRiskFlags,
}

impl DoctorReport {
    pub fn has_data_anomalies(&self) -> bool {
        self.duplicate_count > 0 || self.order_issue_count > 0
    }

    pub fn has_rule_or_parsing_risk(&self) -> bool {
        self.rule_risks.contains_zhang_unit
            || self.rule_risks.missing_large_chinese_number_support
            || !self.suspicious_title_samples.is_empty()
            || !self.long_chapter_samples.is_empty()
    }
}

pub fn analyze_book(book: &Book, rules: &Rules, max_samples: usize) -> DoctorReport {
    let sample_limit = if max_samples == 0 {
        DEFAULT_MAX_SAMPLES
    } else {
        max_samples
    };

    let chapter_numbers: Vec<Option<usize>> = book
        .chapters
        .iter()
        .map(|chapter| extract_chapter_number(&chapter.title, &rules.chapter_number_extraction))
        .collect();

    let numbered: Vec<(usize, &Chapter)> = chapter_numbers
        .iter()
        .enumerate()
        .filter_map(|(idx, number)| number.map(|n| (n, &book.chapters[idx])))
        .collect();

    let unnumbered_chapters = chapter_numbers
        .iter()
        .filter(|number| number.is_none())
        .count();

    let mut seen_numbers: HashMap<usize, &Chapter> = HashMap::new();
    let mut duplicate_count = 0usize;
    let mut duplicate_samples = Vec::new();
    for (number, chapter) in &numbered {
        if let Some(previous) = seen_numbers.get(number) {
            duplicate_count += 1;
            if duplicate_samples.len() < sample_limit {
                duplicate_samples.push(DuplicateIssue {
                    number: *number,
                    first: ChapterSample::from_chapter(previous),
                    second: ChapterSample::from_chapter(chapter),
                });
            }
        }
        seen_numbers.insert(*number, chapter);
    }

    let mut order_issue_count = 0usize;
    let mut order_samples = Vec::new();
    for window in numbered.windows(2) {
        let (previous_number, previous_chapter) = window[0];
        let (current_number, current_chapter) = window[1];
        if current_number <= previous_number {
            order_issue_count += 1;
            if order_samples.len() < sample_limit {
                order_samples.push(OrderIssue {
                    previous_number,
                    previous: ChapterSample::from_chapter(previous_chapter),
                    current_number,
                    current: ChapterSample::from_chapter(current_chapter),
                });
            }
        }
    }

    let suspicious_title_samples = book
        .chapters
        .iter()
        .filter(|chapter| !looks_like_chapter_title(&chapter.title))
        .map(ChapterSample::from_chapter)
        .take(sample_limit)
        .collect();

    let mut long_chapter_samples: Vec<LongChapterIssue> = book
        .chapters
        .iter()
        .filter_map(|chapter| {
            let line_count = chapter
                .end_line
                .saturating_sub(chapter.start_line)
                .saturating_add(1);
            if line_count >= LONG_CHAPTER_LINE_THRESHOLD {
                Some(LongChapterIssue {
                    chapter: ChapterSample::from_chapter(chapter),
                    line_count,
                })
            } else {
                None
            }
        })
        .collect();

    long_chapter_samples.sort_by(|a, b| b.line_count.cmp(&a.line_count));
    long_chapter_samples.truncate(sample_limit);

    let rule_risks = detect_rule_risks(rules);

    DoctorReport {
        total_chapters: book.chapters.len(),
        numbered_chapters: numbered.len(),
        unnumbered_chapters,
        duplicate_count,
        order_issue_count,
        duplicate_samples,
        order_samples,
        suspicious_title_samples,
        long_chapter_samples,
        rule_risks,
    }
}

pub fn build_recommended_commands(
    input: &[PathBuf],
    rules_path: Option<&Path>,
    report: &DoctorReport,
) -> Vec<String> {
    let input_args = shell_join_paths(input);

    let mut commands = Vec::new();

    commands.push(format!(
        "epub-smith --lang zh-CN preview dry-run {input_args}"
    ));

    if report.has_rule_or_parsing_risk() && rules_path.is_none() {
        commands.push(format!(
            "epub-smith --lang zh-CN preview dry-run {input_args} -r <your_rules.toml>"
        ));
    }

    if let Some(path) = rules_path {
        commands.push(format!(
            "epub-smith --lang zh-CN preview dry-run {input_args} -r \"{}\"",
            path.display()
        ));
        commands.push(format!(
            "epub-smith --lang zh-CN convert {input_args} -r \"{}\" -o output.epub",
            path.display()
        ));
    } else {
        commands.push(format!(
            "epub-smith --lang zh-CN convert {input_args} -r <your_rules.toml> -o output.epub"
        ));
    }

    commands
}

fn shell_join_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| format!("\"{}\"", path.display()))
        .collect::<Vec<_>>()
        .join(" ")
}

fn detect_rule_risks(rules: &Rules) -> RuleRiskFlags {
    let extraction_rules: Vec<String> = match &rules.chapter_number_extraction {
        Some(custom) => custom
            .rules
            .iter()
            .map(|rule| rule.pattern.clone())
            .collect(),
        None => ChapterNumberExtraction::default()
            .rules
            .iter()
            .map(|rule| rule.pattern.clone())
            .collect(),
    };

    RuleRiskFlags {
        contains_zhang_unit: rules
            .chapter
            .regex
            .iter()
            .any(|pattern| pattern.contains('张'))
            || extraction_rules
                .iter()
                .any(|pattern| pattern.contains('张')),
        missing_large_chinese_number_support: chinese_number_pattern_is_limited(
            &rules.chapter.regex,
            &extraction_rules,
        ),
    }
}

fn chinese_number_pattern_is_limited(
    chapter_patterns: &[String],
    extraction_patterns: &[String],
) -> bool {
    let chapter_has_chinese_unit = chapter_patterns.iter().any(|pattern| {
        pattern.contains('章')
            || pattern.contains('节')
            || pattern.contains('卷')
            || pattern.contains('回')
    });
    let extraction_has_chinese_unit = extraction_patterns.iter().any(|pattern| {
        pattern.contains('章')
            || pattern.contains('节')
            || pattern.contains('卷')
            || pattern.contains('回')
    });

    if !chapter_has_chinese_unit && !extraction_has_chinese_unit {
        return false;
    }

    let chapter_supports_large = chapter_patterns
        .iter()
        .any(|pattern| pattern_supports_large_numbers(pattern));
    let extraction_supports_large = extraction_patterns
        .iter()
        .any(|pattern| pattern_supports_large_numbers(pattern));

    !chapter_supports_large || !extraction_supports_large
}

fn pattern_supports_large_numbers(pattern: &str) -> bool {
    ["百", "千", "万", "零", "〇", "两"]
        .iter()
        .any(|token| pattern.contains(token))
}

fn looks_like_chapter_title(title: &str) -> bool {
    let normalized = title.trim_start();
    if normalized.is_empty() {
        return false;
    }

    if regex::Regex::new(
        r"^(?:第)?[零〇一二三四五六七八九十百千万两0-9]+[章节卷节回](?:\s|[:：\-—]|$)",
    )
    .map(|re| re.is_match(normalized))
    .unwrap_or(false)
    {
        return true;
    }

    let known_prefixes = [
        "第", "Chapter ", "Section ", "Part ", "番外", "序", "尾声", "后记", "附录", "正文",
    ];

    known_prefixes
        .iter()
        .any(|prefix| normalized.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Book, Chapter, Meta, Rules};

    fn chapter(title: &str, start_line: usize, end_line: usize) -> Chapter {
        Chapter {
            title: title.to_string(),
            paragraphs: Vec::new(),
            slug: title.to_string(),
            start_line,
            end_line,
        }
    }

    fn empty_book(chapters: Vec<Chapter>) -> Book {
        Book {
            meta: Meta::default(),
            chapters,
        }
    }

    #[test]
    fn detects_duplicate_and_order_issues() {
        let book = empty_book(vec![
            chapter("第一章 开始", 0, 10),
            chapter("第二章 发展", 11, 20),
            chapter("第二章 重复", 21, 30),
            chapter("第一章 回退", 31, 40),
        ]);

        let report = analyze_book(&book, &Rules::default(), 5);
        assert_eq!(report.duplicate_count, 2);
        assert_eq!(report.order_issue_count, 2);
        assert!(!report.duplicate_samples.is_empty());
        assert!(!report.order_samples.is_empty());
    }

    #[test]
    fn default_rules_no_longer_have_zhang_or_large_number_risks() {
        let report = analyze_book(
            &empty_book(vec![chapter("第一章", 0, 5)]),
            &Rules::default(),
            5,
        );
        assert!(!report.rule_risks.contains_zhang_unit);
        assert!(!report.rule_risks.missing_large_chinese_number_support);
    }

    #[test]
    fn default_rules_treat_numeric_chapter_titles_as_non_suspicious() {
        let report = analyze_book(
            &empty_book(vec![chapter("1225章 发展", 0, 5)]),
            &Rules::default(),
            5,
        );
        assert!(report.suspicious_title_samples.is_empty());
    }

    #[test]
    fn detects_suspicious_non_chapter_titles() {
        let book = empty_book(vec![
            chapter("第一章 正常", 0, 10),
            chapter("一张张截图，迅速密集发出来。", 11, 20),
        ]);
        let report = analyze_book(&book, &Rules::default(), 5);
        assert_eq!(report.suspicious_title_samples.len(), 1);
    }
}
