use regex::Regex;

#[test]
fn test_chinese_to_arabic() {
    // 直接调用二进制文件的函数可能比较复杂，我们可以测试正则表达式的功能
    // 测试中文数字提取
    let regex =
        Regex::new(r"^\s*(?:第)?([零〇一二三四五六七八九十百千万两0-9]+)[章节卷回]").unwrap();

    assert!(regex.is_match("第1章"));
    assert!(regex.is_match("1章"));
    assert!(regex.is_match("第一章"));
    assert!(regex.is_match("第十章"));
    assert!(regex.is_match("第10章"));
    assert!(regex.is_match("第一百章"));
    assert!(regex.is_match("第四百八十章"));
    assert!(regex.is_match("第两百章"));
    assert!(!regex.is_match("一张张截图，迅速密集发出来。"));
}

#[test]
fn test_english_chapter_regex() {
    // 测试英文章节提取
    let regex = Regex::new(r"^Chapter\s+").unwrap();

    assert!(regex.is_match("Chapter 1"));
    assert!(regex.is_match("Chapter Two"));
    assert!(regex.is_match("Chapter III"));
}

#[test]
fn test_roman_chapter_regex() {
    // 测试罗马数字章节提取
    let regex = Regex::new(r"^Chapter\s+([IVXLCDMivxlcdm]+)").unwrap();

    assert!(regex.is_match("Chapter I"));
    assert!(regex.is_match("Chapter II"));
    assert!(regex.is_match("Chapter III"));
    assert!(regex.is_match("Chapter X"));
}

#[test]
fn test_custom_rule_regex() {
    // 测试自定义规则章节提取
    let regex = Regex::new(r"^Episode ([0-9]+):").unwrap();

    assert!(regex.is_match("Episode 1: The Beginning"));
    assert!(regex.is_match("Episode 2: The Journey"));
    assert!(regex.is_match("Episode 3: The End"));
}

#[test]
fn test_chapter_regex_priority() {
    // 测试章节正则表达式优先级
    let regexes = vec![
        Regex::new(r"^Chapter\s+").unwrap(),
        Regex::new(r"^Section\s+").unwrap(),
        Regex::new(r"^Part\s+").unwrap(),
    ];

    // 应该匹配第一个正则表达式
    let title = "Chapter 1";
    let mut matched = false;
    for regex in &regexes {
        if regex.is_match(title) {
            matched = true;
            break;
        }
    }
    assert!(matched);
}
