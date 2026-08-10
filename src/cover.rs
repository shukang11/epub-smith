use crate::models::Meta;
use anyhow::{Context, Result};
use std::path::Path;

/// 封面模板（编译期内嵌，template export 可导出编辑）
const COVER_TEMPLATE: &str = include_str!("templates/cover.svg");
const TITLE_PLACEHOLDER: &str = "{{ title_lines }}";
const AUTHOR_PLACEHOLDER: &str = "{{ author }}";
/// 书名单行最大字符数
const MAX_TITLE_CHARS: usize = 14;
/// 书名最多行数
const MAX_TITLE_LINES: usize = 3;
/// 封面渲染尺寸（与模板 viewBox 一致）
const COVER_WIDTH: u32 = 1600;
const COVER_HEIGHT: u32 = 2560;
/// 书名行距与首行纵坐标（与模板装饰线 980/1460 相配合）
const TITLE_LINE_HEIGHT: i32 = 130;
const TITLE_FONT_FAMILY: &str =
    "PingFang SC, Songti SC, Microsoft YaHei, SimSun, Noto Sans CJK SC, sans-serif";

/// XML 转义（书名/作者可能含 `& < > " '`）
fn escape_xml(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&apos;".to_string(),
            c => c.to_string(),
        })
        .collect()
}

/// 将书名切分为最多 3 行（每行不超过 `MAX_TITLE_CHARS` 字符）
fn split_title_lines(title: &str) -> Vec<String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return vec!["Untitled".to_string()];
    }
    let chars: Vec<char> = trimmed.chars().collect();
    let mut lines = Vec::new();
    let mut i = 0;
    while i < chars.len() && lines.len() < MAX_TITLE_LINES {
        let end = (i + MAX_TITLE_CHARS).min(chars.len());
        lines.push(chars[i..end].iter().collect());
        i = end;
    }
    lines
}

/// 根据行数生成书名 SVG 文本块（先切行后转义，避免切断 XML 实体）
fn build_title_svg(title: &str) -> String {
    let lines = split_title_lines(title);
    let start_y = match lines.len() {
        1 => 1200,
        2 => 1120,
        _ => 1040,
    };
    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        let y = start_y + i as i32 * TITLE_LINE_HEIGHT;
        out.push_str(&format!(
            "  <text x=\"800\" y=\"{y}\" font-family=\"{TITLE_FONT_FAMILY}\" font-size=\"120\" fill=\"#ffffff\" text-anchor=\"middle\">{}</text>\n",
            escape_xml(line)
        ));
    }
    out
}

/// 用书名/作者填充封面 SVG 模板
fn render_svg(meta: &Meta) -> Result<String> {
    let svg = COVER_TEMPLATE
        .replace(TITLE_PLACEHOLDER, &build_title_svg(&meta.title))
        .replace(AUTHOR_PLACEHOLDER, &escape_xml(&meta.author));
    if svg.contains("{{") {
        anyhow::bail!("Cover template contains unresolved placeholders");
    }
    Ok(svg)
}

/// 生成默认封面 PNG（离线 SVG 栅格化，无网络/模型依赖）
pub fn generate_cover(meta: &Meta, output_path: &Path) -> Result<()> {
    let svg = render_svg(meta)?;

    // 解析 SVG（含系统字体库，用于渲染中文）
    let mut opt = resvg::usvg::Options::default();
    std::sync::Arc::make_mut(&mut opt.fontdb).load_system_fonts();
    let tree = resvg::usvg::Tree::from_str(&svg, &opt)
        .with_context(|| "Failed to parse cover SVG template")?;

    // 渲染为像素图
    let mut pixmap = resvg::tiny_skia::Pixmap::new(COVER_WIDTH, COVER_HEIGHT)
        .context("Failed to allocate cover pixmap")?;
    resvg::render(
        &tree,
        resvg::usvg::Transform::default(),
        &mut pixmap.as_mut(),
    );

    // 编码为 PNG 并写出
    let png = pixmap.encode_png().context("Failed to encode cover PNG")?;
    std::fs::write(output_path, png)
        .with_context(|| format!("Failed to write cover image: {}", output_path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(title: &str, author: &str) -> Meta {
        Meta {
            title: title.to_string(),
            author: author.to_string(),
            language: "zh-CN".to_string(),
            identifier: "urn:uuid:test".to_string(),
            modified: "2026-01-01T00:00:00Z".to_string(),
            cover: None,
        }
    }

    #[test]
    fn escape_xml_escapes_special_chars() {
        assert_eq!(escape_xml("A&B<C>"), "A&amp;B&lt;C&gt;");
        assert_eq!(escape_xml("引\"号'"), "引&quot;号&apos;");
        assert_eq!(escape_xml("正常文本"), "正常文本");
    }

    #[test]
    fn split_title_respects_max_lines_and_chars() {
        let lines = split_title_lines("第一章 开始");
        assert_eq!(lines, vec!["第一章 开始"]);

        let long = "这是一个非常长的书名超过十四个字了怎么办";
        let lines = split_title_lines(long);
        assert!(lines.len() <= MAX_TITLE_LINES);
        for line in &lines {
            assert!(line.chars().count() <= MAX_TITLE_CHARS);
        }
        // 切行按字符边界，不切断多字节字符
        assert_eq!(split_title_lines(""), vec!["Untitled"]);
    }

    #[test]
    fn render_svg_replaces_all_placeholders() {
        let m = meta("A&B", "某作者");
        let svg = render_svg(&m).unwrap();
        assert!(!svg.contains("{{"), "占位符未全部替换");
        assert!(svg.contains("A&amp;B"));
        assert!(svg.contains("<text"));
        assert!(svg.contains("某作者"));
    }

    #[test]
    fn generate_cover_produces_png_of_expected_size() {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("cover.png");
        generate_cover(&meta("测试书名", "作者"), &out).unwrap();

        let bytes = std::fs::read(&out).unwrap();
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n", "不是合法 PNG");
        // PNG IHDR：签名 8 字节 + 长度 4 字节 + "IHDR" 4 字节后是宽高（大端）
        let w = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let h = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
        assert_eq!((w, h), (COVER_WIDTH, COVER_HEIGHT));
    }
}
