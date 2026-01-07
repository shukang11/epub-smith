use epub_smith::models::{ChapterNumberExtraction, ChapterNumberRule};
use epub_smith::{
    convert_chinese_to_arabic, convert_english_to_arabic, convert_number, convert_roman_to_arabic,
    extract_chapter_number,
};

#[test]
fn test_chinese_number_conversion() {
    // 测试阿拉伯数字直接转换
    assert_eq!(convert_chinese_to_arabic("1"), Some(1));
    assert_eq!(convert_chinese_to_arabic("10"), Some(10));
    assert_eq!(convert_chinese_to_arabic("100"), Some(100));

    // 测试简单中文数字
    assert_eq!(convert_chinese_to_arabic("一"), Some(1));
    assert_eq!(convert_chinese_to_arabic("二"), Some(2));
    assert_eq!(convert_chinese_to_arabic("九"), Some(9));

    // 测试复杂中文数字
    assert_eq!(convert_chinese_to_arabic("十"), Some(10));
    assert_eq!(convert_chinese_to_arabic("十一"), Some(11));
    assert_eq!(convert_chinese_to_arabic("二十"), Some(20));
    assert_eq!(convert_chinese_to_arabic("三十一"), Some(31));
    assert_eq!(convert_chinese_to_arabic("一百"), Some(100));
    assert_eq!(convert_chinese_to_arabic("一百零一"), Some(101));
    assert_eq!(convert_chinese_to_arabic("二百三十"), Some(230));
    assert_eq!(convert_chinese_to_arabic("一千二百三十四"), Some(1234));

    // 测试无法识别的中文数字
    assert_eq!(convert_chinese_to_arabic("未知"), None);
    assert_eq!(convert_chinese_to_arabic("abc"), None);
}

#[test]
fn test_english_number_conversion() {
    // 测试阿拉伯数字直接转换
    assert_eq!(convert_english_to_arabic("1"), Some(1));
    assert_eq!(convert_english_to_arabic("10"), Some(10));

    // 测试英文单词数字
    assert_eq!(convert_english_to_arabic("one"), Some(1));
    assert_eq!(convert_english_to_arabic("One"), Some(1));
    assert_eq!(convert_english_to_arabic("ONE"), Some(1));
    assert_eq!(convert_english_to_arabic("two"), Some(2));
    assert_eq!(convert_english_to_arabic("nine"), Some(9));
    assert_eq!(convert_english_to_arabic("ten"), Some(10));

    // 测试无法识别的英文数字
    assert_eq!(convert_english_to_arabic("eleven"), None);
    assert_eq!(convert_english_to_arabic("abc"), None);
}

#[test]
fn test_roman_number_conversion() {
    // 测试基本罗马数字
    assert_eq!(convert_roman_to_arabic("I"), Some(1));
    assert_eq!(convert_roman_to_arabic("V"), Some(5));
    assert_eq!(convert_roman_to_arabic("X"), Some(10));
    assert_eq!(convert_roman_to_arabic("L"), Some(50));
    assert_eq!(convert_roman_to_arabic("C"), Some(100));
    assert_eq!(convert_roman_to_arabic("D"), Some(500));
    assert_eq!(convert_roman_to_arabic("M"), Some(1000));

    // 测试组合罗马数字
    assert_eq!(convert_roman_to_arabic("II"), Some(2));
    assert_eq!(convert_roman_to_arabic("III"), Some(3));
    assert_eq!(convert_roman_to_arabic("IV"), Some(4));
    assert_eq!(convert_roman_to_arabic("VI"), Some(6));
    assert_eq!(convert_roman_to_arabic("IX"), Some(9));
    assert_eq!(convert_roman_to_arabic("XII"), Some(12));
    assert_eq!(convert_roman_to_arabic("XXI"), Some(21));
    assert_eq!(convert_roman_to_arabic("XLV"), Some(45));
    assert_eq!(convert_roman_to_arabic("XC"), Some(90));
    assert_eq!(convert_roman_to_arabic("CD"), Some(400));
    assert_eq!(convert_roman_to_arabic("CM"), Some(900));
    assert_eq!(convert_roman_to_arabic("MCMXCIV"), Some(1994));

    // 测试小写罗马数字
    assert_eq!(convert_roman_to_arabic("i"), Some(1));
    assert_eq!(convert_roman_to_arabic("ii"), Some(2));
    assert_eq!(convert_roman_to_arabic("iii"), Some(3));
    assert_eq!(convert_roman_to_arabic("iv"), Some(4));
    assert_eq!(convert_roman_to_arabic("v"), Some(5));

    // 测试无法识别的罗马数字
    assert_eq!(convert_roman_to_arabic("A"), None);
    assert_eq!(convert_roman_to_arabic("abc"), None);
}

#[test]
fn test_unified_number_conversion() {
    // 测试阿拉伯数字转换
    assert_eq!(convert_number("1", "arabic"), Some(1));
    assert_eq!(convert_number("10", "arabic"), Some(10));

    // 测试中文数字转换
    assert_eq!(convert_number("一", "chinese"), Some(1));
    assert_eq!(convert_number("十", "chinese"), Some(10));

    // 测试英文数字转换
    assert_eq!(convert_number("one", "english"), Some(1));
    assert_eq!(convert_number("ten", "english"), Some(10));

    // 测试罗马数字转换
    assert_eq!(convert_number("I", "roman"), Some(1));
    assert_eq!(convert_number("X", "roman"), Some(10));

    // 测试自动识别
    assert_eq!(convert_number("1", "auto"), Some(1));
    assert_eq!(convert_number("一", "auto"), Some(1));
    assert_eq!(convert_number("one", "auto"), Some(1));
    assert_eq!(convert_number("I", "auto"), Some(1));

    // 测试无法识别的数字
    assert_eq!(convert_number("unknown", "auto"), None);
    assert_eq!(convert_number("abc", "arabic"), None);
}

#[test]
fn test_chapter_number_extraction() {
    // 测试默认规则下的中文数字提取
    assert_eq!(extract_chapter_number("第1章", &None), Some(1));
    assert_eq!(extract_chapter_number("1章", &None), Some(1));
    assert_eq!(extract_chapter_number("第一章", &None), Some(1));
    assert_eq!(extract_chapter_number("第十章", &None), Some(10));
    assert_eq!(extract_chapter_number("第10章", &None), Some(10));
    assert_eq!(extract_chapter_number("第1节", &None), Some(1));
    assert_eq!(extract_chapter_number("第1卷", &None), Some(1));
    assert_eq!(extract_chapter_number("第1张", &None), Some(1));

    // 测试默认规则下的英文数字提取
    assert_eq!(extract_chapter_number("Chapter 1", &None), Some(1));
    assert_eq!(extract_chapter_number("Chapter Two", &None), Some(2));
    assert_eq!(extract_chapter_number("Section 1", &None), Some(1));

    // 测试默认规则下的罗马数字提取
    assert_eq!(extract_chapter_number("Chapter I", &None), Some(1));
    assert_eq!(extract_chapter_number("Chapter II", &None), Some(2));
    assert_eq!(extract_chapter_number("Section III", &None), Some(3));

    // 测试无法提取章节号的情况
    assert_eq!(extract_chapter_number("序", &None), None);
    assert_eq!(extract_chapter_number("番外：某件小事", &None), None);
    assert_eq!(extract_chapter_number("尾声", &None), None);
}

#[test]
fn test_custom_extraction_rules() {
    // 创建自定义规则
    let custom_rules = ChapterNumberExtraction {
        rules: vec![
            ChapterNumberRule {
                pattern: "^Episode ([0-9]+):".to_string(),
                capture_group: 1,
                number_type: "arabic".to_string(),
            },
            ChapterNumberRule {
                pattern: "^Special ([0-9]+):".to_string(),
                capture_group: 1,
                number_type: "arabic".to_string(),
            },
        ],
    };

    // 测试自定义规则
    assert_eq!(
        extract_chapter_number("Episode 1: The Beginning", &Some(custom_rules.clone())),
        Some(1)
    );
    assert_eq!(
        extract_chapter_number("Episode 2: The Journey", &Some(custom_rules.clone())),
        Some(2)
    );
    assert_eq!(
        extract_chapter_number("Special 1: Bonus Content", &Some(custom_rules.clone())),
        Some(1)
    );

    // 测试自定义规则无法匹配的情况
    assert_eq!(
        extract_chapter_number("Chapter 1", &Some(custom_rules.clone())),
        None
    );
    assert_eq!(
        extract_chapter_number("第1章", &Some(custom_rules.clone())),
        None
    );
}

#[test]
fn test_mixed_chapter_formats() {
    // 测试混合章节格式
    let extraction_rules = None;

    // 测试各种格式的章节号提取
    assert_eq!(extract_chapter_number("第1章", &extraction_rules), Some(1));
    assert_eq!(
        extract_chapter_number("Chapter 1", &extraction_rules),
        Some(1)
    );
    assert_eq!(
        extract_chapter_number("Chapter I", &extraction_rules),
        Some(1)
    );
    assert_eq!(
        extract_chapter_number("Section 1", &extraction_rules),
        Some(1)
    );
    assert_eq!(extract_chapter_number("Part 1", &extraction_rules), Some(1));
}
