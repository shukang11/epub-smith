/// 将中文数字转换为阿拉伯数字（支持复杂数字，如一百二十三）
pub fn convert_chinese_to_arabic(chinese_num: &str) -> Option<usize> {
    // 尝试直接解析为阿拉伯数字
    if let Ok(num) = chinese_num.parse::<usize>() {
        return Some(num);
    }

    // 定义基本中文数字映射
    let basic_map = [
        ("零", 0),
        ("一", 1),
        ("二", 2),
        ("三", 3),
        ("四", 4),
        ("五", 5),
        ("六", 6),
        ("七", 7),
        ("八", 8),
        ("九", 9),
    ];

    let unit_map = [("十", 10), ("百", 100), ("千", 1000), ("万", 10000)];

    // 处理单个数字
    for (word, num) in basic_map.iter() {
        if chinese_num == *word {
            return Some(*num);
        }
    }

    // 处理十、百、千、万开头的情况
    for (word, unit) in unit_map.iter() {
        if chinese_num == *word {
            return Some(*unit);
        }
    }

    // 处理复合数字，如十、十一、二十、三十一、一百、一百零一、二百三十、一千二百三十四
    // 正确的处理逻辑应该是从左到右遍历
    let mut result = 0;
    let mut temp = 0;

    for c in chinese_num.chars() {
        // 处理基本数字
        if let Some((_, num)) = basic_map.iter().find(|(word, _)| word.starts_with(c)) {
            temp = *num;
        }
        // 处理零
        else if c == '零' {
            // 零表示占位，跳过
            continue;
        }
        // 处理单位
        else if let Some((_, unit)) = unit_map.iter().find(|(word, _)| word.starts_with(c)) {
            if temp == 0 {
                // 单位前面没有数字，如十、百、千、万
                temp = 1;
            }
            result += temp * unit;
            temp = 0;
        }
        // 无法识别的字符
        else {
            return None;
        }
    }

    // 处理最后剩下的数字，如十一中的"一"，二十中的"二"
    if temp > 0 {
        result += temp;
    }

    Some(result)
}

/// 将英文数字转换为阿拉伯数字
pub fn convert_english_to_arabic(english_num: &str) -> Option<usize> {
    // 基本英文数字映射
    let num_map = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
    ];

    // 尝试直接解析为阿拉伯数字
    if let Ok(num) = english_num.parse::<usize>() {
        return Some(num);
    }

    // 转换为小写并尝试匹配
    let lowercase_num = english_num.to_lowercase();
    for (word, num) in num_map.iter() {
        if *word == lowercase_num {
            return Some(*num);
        }
    }

    None
}

/// 将罗马数字转换为阿拉伯数字
pub fn convert_roman_to_arabic(roman_num: &str) -> Option<usize> {
    let roman_map = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ];

    let mut result = 0;
    let mut prev = 0;

    for c in roman_num.chars().rev() {
        let current = roman_map
            .iter()
            .find(|(r, _)| *r == c.to_ascii_uppercase())
            .map(|(_, v)| *v)?;

        if current < prev {
            result -= current;
        } else {
            result += current;
        }

        prev = current;
    }

    Some(result)
}

/// 阿拉伯数字直接转换（用于类型统一）
pub fn convert_arabic_to_arabic(num_str: &str) -> Option<usize> {
    num_str.parse::<usize>().ok()
}

/// 统一数字转换函数，支持自动识别数字类型
pub fn convert_number(num_str: &str, number_type: &str) -> Option<usize> {
    match number_type {
        "auto" => {
            // 自动识别数字类型
            convert_arabic_to_arabic(num_str)
                .or_else(|| convert_chinese_to_arabic(num_str))
                .or_else(|| convert_english_to_arabic(num_str))
                .or_else(|| convert_roman_to_arabic(num_str))
        }
        "arabic" => convert_arabic_to_arabic(num_str),
        "chinese" => convert_chinese_to_arabic(num_str),
        "english" => convert_english_to_arabic(num_str),
        "roman" => convert_roman_to_arabic(num_str),
        _ => None,
    }
}

/// 从章节标题中提取序号
pub fn extract_chapter_number(
    title: &str,
    extraction_rules: &Option<crate::models::ChapterNumberExtraction>,
) -> Option<usize> {
    use regex::Regex;

    // 如果没有配置自定义规则，使用默认规则
    let rules = match extraction_rules {
        Some(rules) => &rules.rules,
        None => &crate::models::ChapterNumberExtraction::default().rules,
    };

    // 按优先级尝试匹配规则
    for rule in rules {
        if let Ok(regex) = Regex::new(&rule.pattern)
            && let Some(captures) = regex.captures(title)
            && let Some(num_str) = captures.get(rule.capture_group)
            && let Some(num) = convert_number(num_str.as_str(), &rule.number_type)
        {
            return Some(num);
        }
    }

    None
}
