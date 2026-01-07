use lazy_static::lazy_static;
use regex::Regex;

// 编译一次正则表达式，避免重复编译
lazy_static! {
    // 1. 修复 <ahref...> 这种标签名和属性之间缺少空格的情况
    pub static ref RE_TAG_SPACE: Regex = Regex::new(r#"<([a-zA-Z][a-zA-Z0-9]*)(href|src|target|alt|width|height|title|class|id|bl_id|a_id|b_id)"#).unwrap();
    // 2. 修复 hrefhttp:// 这种缺少等号和引号的情况
    pub static ref RE_HREF: Regex = Regex::new(r#"href(http[s]?://[^">]+)"#).unwrap();
    // 3. 修复 srchttp:// 这种缺少等号和引号的情况
    pub static ref RE_SRC: Regex = Regex::new(r#"src(http[s]?://[^">]+)"#).unwrap();
    // 4. 修复 target_blank 这种缺少等号和引号的情况，保留下划线
    pub static ref RE_ATTR_UNDERSCORE: Regex = Regex::new(r#"(target|alt|title|class|id|bl_id|a_id|b_id|width|height)_([^\s">]+)"#).unwrap();
    // 5. 修复 bl_id1234 这种缺少等号和引号的情况
    pub static ref RE_ATTR_NUM: Regex = Regex::new(r#"(bl_id|a_id|b_id|width|height)(\d+)"#).unwrap();
    // 6. 修复属性之间缺少空格的问题（如 target="_blank"href="..."）
    pub static ref RE_ATTR_NO_SPACE: Regex = Regex::new(r#""([a-zA-Z][a-zA-Z0-9_]*)="#).unwrap();
    // 7. 清理属性值前面的多余空格（如 href=" http://）
    pub static ref RE_ATTR_LEADING_SPACE: Regex = Regex::new(r#"="\s+"#).unwrap();
    // 8. 清理多余的空格
    pub static ref RE_EXTRA_SPACE: Regex = Regex::new(r#"\s+">"#).unwrap();
    // 9. 修复引号之间的多余空格
    pub static ref RE_QUOTE_SPACE: Regex = Regex::new(r#""\s+""#).unwrap();
}

/// 转义HTML特殊字符，只转义文本内容，不影响HTML标签
pub fn escape_html(content: &str) -> String {
    let mut escaped_line = String::with_capacity(content.len() * 2); // 预分配空间减少扩容
    let mut in_tag = false;
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        // 过滤无效的XML字符
        if c.is_control() && !matches!(c, '\n' | '\r' | '\t') {
            continue;
        }

        if in_tag {
            // 在标签内，直接添加字符
            escaped_line.push(c);
            // 检查是否结束标签
            if c == '>' {
                in_tag = false;
            }
        } else {
            match c {
                '<' => {
                    // 检查是否是HTML标签的开始
                    if let Some(next_char) = chars.peek() {
                        if next_char.is_alphabetic() || *next_char == '/' {
                            // 是HTML标签，不转义
                            escaped_line.push(c);
                            in_tag = true;
                        } else {
                            // 不是HTML标签，转义
                            escaped_line.push_str("&lt;");
                        }
                    } else {
                        // 行尾的<，转义
                        escaped_line.push_str("&lt;");
                    }
                }
                '>' => {
                    // 转义>为&gt;
                    escaped_line.push_str("&gt;");
                }
                '&' => {
                    // 转义&为&amp;
                    escaped_line.push_str("&amp;");
                }
                _ => {
                    // 其他字符直接添加
                    escaped_line.push(c);
                }
            }
        }
    }

    escaped_line
}

/// 修复错误格式的HTML标签
pub fn fix_html_tags(content: &str) -> String {
    let mut fixed_line = content.to_string();

    // 1. 修复 <ahref...> 这种标签名和属性之间缺少空格的情况
    fixed_line = RE_TAG_SPACE
        .replace_all(&fixed_line, r#"<$1 $2"#)
        .to_string();

    // 2. 修复 hrefhttp:// 这种缺少等号和引号的情况
    fixed_line = RE_HREF.replace_all(&fixed_line, r#"href="$1""#).to_string();

    // 3. 修复 srchttp:// 这种缺少等号和引号的情况
    fixed_line = RE_SRC.replace_all(&fixed_line, r#"src="$1""#).to_string();

    // 4. 修复 target_blank 这种缺少等号和引号的情况，保留下划线
    fixed_line = RE_ATTR_UNDERSCORE
        .replace_all(&fixed_line, r#"$1="$2""#)
        .to_string();

    // 5. 修复 bl_id1234 这种缺少等号和引号的情况
    fixed_line = RE_ATTR_NUM
        .replace_all(&fixed_line, r#"$1="$2""#)
        .to_string();

    // 11. 修复属性之间缺少空格的问题（如 target="_blank"href="..."）
    fixed_line = RE_ATTR_NO_SPACE
        .replace_all(&fixed_line, r#"" $1="#)
        .to_string();

    // 12. 清理属性值前面的多余空格（如 href=" http://）
    fixed_line = RE_ATTR_LEADING_SPACE
        .replace_all(&fixed_line, r#"="#)
        .to_string();

    // 13. 清理多余的空格
    fixed_line = RE_EXTRA_SPACE.replace_all(&fixed_line, r#">"#).to_string();

    // 14. 修复引号之间的多余空格
    fixed_line = RE_QUOTE_SPACE.replace_all(&fixed_line, r#"""#).to_string();

    fixed_line
}
