use chardet::detect;
use encoding_rs::{Encoding, UTF_8};

/// 探测原始内容的编码
pub fn detect_encoding(raw_content: &[u8]) -> &'static str {
    let (charset, confidence, _language) = detect(raw_content);

    // 如果置信度足够高，使用探测到的编码，否则使用UTF-8
    if confidence > 0.8 {
        // 根据charset字符串返回对应的编码名称
        match charset.as_str() {
            "UTF-8" => "UTF-8",
            "GB2312" => "GBK",
            "GBK" => "GBK",
            "GB18030" => "GB18030",
            "Big5" => "Big5",
            "Shift_JIS" => "Shift_JIS",
            "EUC-JP" => "EUC-JP",
            "ISO-8859-1" => "ISO-8859-1",
            "Windows-1252" => "Windows-1252",
            _ => "UTF-8",
        }
    } else {
        "UTF-8"
    }
}

/// 将原始内容转换为UTF-8字符串
pub fn convert_to_utf8(raw_content: &[u8], encoding: Option<&str>) -> String {
    match encoding {
        Some(enc) => {
            // 使用指定的编码
            if let Some(encoding) = Encoding::for_label(enc.as_bytes()) {
                let (cow, _, _) = encoding.decode(raw_content);
                cow.to_string()
            } else {
                // 如果指定的编码无效，回退到自动探测
                auto_detect_and_convert(raw_content)
            }
        }
        None => {
            // 自动探测编码
            auto_detect_and_convert(raw_content)
        }
    }
}

/// 自动探测编码并转换为UTF-8
fn auto_detect_and_convert(raw_content: &[u8]) -> String {
    let detected_encoding = detect_encoding(raw_content);
    let encoding = Encoding::for_label(detected_encoding.as_bytes()).unwrap_or(UTF_8);

    let (cow, _, _) = encoding.decode(raw_content);
    cow.to_string()
}
