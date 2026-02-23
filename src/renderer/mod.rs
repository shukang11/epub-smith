use anyhow::{Context, Result};
use rust_i18n::t;
use std::path::PathBuf;
use tempfile::tempdir;
use tera::{Context as TeraContext, Tera};

use crate::{
    config::Config,
    models::{Book, Chapter},
};

/// 将书籍渲染为XHTML文件
pub fn render_book(book: &Book, config: &Config) -> Result<Vec<PathBuf>> {
    // 初始化Tera模板引擎
    let tera = initialize_tera()?;

    // 创建临时目录用于存储XHTML文件
    let temp_dir = tempdir().with_context(|| "Failed to create temporary directory")?;

    // 获取临时目录路径并阻止自动删除
    let temp_path = temp_dir.path().to_path_buf();
    let _ = temp_dir.keep();

    let mut xhtml_files = Vec::new();

    // 渲染每个章节
    for (i, chapter) in book.chapters.iter().enumerate() {
        let xhtml_path = temp_path.join(format!("chapter_{:03}.xhtml", i + 1));
        render_chapter(chapter, i + 1, book, &tera, &xhtml_path)?;
        xhtml_files.push(xhtml_path);
    }

    // 渲染导航文件
    let nav_path = temp_path.join("nav.xhtml");
    render_nav(book, &tera, &nav_path)?;
    xhtml_files.push(nav_path);

    // 渲染CSS文件
    let css_path = temp_path.join("style.css");
    let _css_content = render_css(&css_path, &config.style)?;
    xhtml_files.push(css_path);

    // 渲染OPF文件
    let opf_path = temp_path.join("content.opf");
    render_opf(book, &xhtml_files, &tera, &opf_path)?;
    xhtml_files.push(opf_path);

    // 渲染mimetype文件
    let mimetype_path = temp_path.join("mimetype");
    render_mimetype(&mimetype_path)?;
    xhtml_files.push(mimetype_path);

    // 渲染container.xml文件
    let container_path = temp_path.join("META-INF/container.xml");
    std::fs::create_dir_all(temp_path.join("META-INF"))?;
    render_container(&container_path)?;
    xhtml_files.push(container_path);

    // 处理封面图片
    if let Some(cover_path) = &book.meta.cover {
        let cover_ext = cover_path
            .extension()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("png");

        let cover_filename = format!("cover.{cover_ext}");
        let dest_cover_path = temp_path.join(&cover_filename);
        std::fs::copy(cover_path, &dest_cover_path).with_context(|| {
            format!(
                "Failed to copy cover image from {} to {}",
                cover_path.display(),
                dest_cover_path.display()
            )
        })?;
        xhtml_files.push(dest_cover_path);
    }

    Ok(xhtml_files)
}

/// 初始化Tera模板引擎
pub fn initialize_tera() -> Result<Tera> {
    // 创建Tera实例，添加所有模板
    let mut tera = Tera::default();

    // 添加章节模板
    tera.add_raw_template(
        "chapter.xhtml",
        include_str!("../templates/chapter.xhtml.tera"),
    )?;

    // 添加导航模板
    tera.add_raw_template("nav.xhtml", include_str!("../templates/nav.xhtml.tera"))?;

    // 添加OPF模板
    tera.add_raw_template("content.opf", include_str!("../templates/content.opf.tera"))?;

    Ok(tera)
}

/// 渲染单个章节为XHTML文件
pub fn render_chapter(
    chapter: &Chapter,
    chapter_num: usize,
    book: &Book,
    tera: &Tera,
    output_path: &PathBuf,
) -> Result<()> {
    let mut context = TeraContext::new();
    context.insert("chapter", chapter);
    context.insert("chapter_num", &chapter_num);
    context.insert("book", book);

    let rendered = tera.render("chapter.xhtml", &context)?;
    std::fs::write(output_path, rendered)?;

    Ok(())
}

/// 渲染导航文件
pub fn render_nav(book: &Book, tera: &Tera, output_path: &PathBuf) -> Result<()> {
    let mut context = TeraContext::new();
    context.insert("book", book);

    let rendered = tera.render("nav.xhtml", &context)?;
    std::fs::write(output_path, rendered)?;

    Ok(())
}

/// 渲染OPF文件
pub fn render_opf(
    book: &Book,
    xhtml_files: &[PathBuf],
    tera: &Tera,
    output_path: &PathBuf,
) -> Result<()> {
    let mut context = TeraContext::new();
    context.insert("book", book);

    // 收集所有XHTML文件的文件名
    let xhtml_filenames: Vec<String> = xhtml_files
        .iter()
        .filter(|path| path.extension().map(|ext| ext == "xhtml").unwrap_or(false))
        .filter_map(|path| path.file_name().and_then(|os_str| os_str.to_str()))
        .map(|filename| filename.to_string())
        .collect();

    context.insert("xhtml_files", &xhtml_filenames);

    // 处理封面文件名
    let cover_item = if let Some(cover_path) = &book.meta.cover {
        let cover_ext = cover_path
            .extension()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("png");

        let media_type = match cover_ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            _ => "image/png",
        };

        let filename = format!("cover.{cover_ext}");
        Some(format!(
            r#"        <item href="{filename}" id="cover-image" media-type="{media_type}" properties="cover-image" />"#
        ))
    } else {
        None
    };
    context.insert("cover_item", &cover_item);

    let rendered = tera.render("content.opf", &context)?;
    std::fs::write(output_path, rendered)?;

    Ok(())
}

/// 渲染CSS文件
pub fn render_css(output_path: &PathBuf, custom_css: &Option<PathBuf>) -> Result<String> {
    // 尝试加载自定义CSS，如果失败则使用默认CSS
    let css_content = if let Some(css_path) = custom_css {
        match std::fs::read_to_string(css_path) {
            Ok(content) => {
                eprintln!("{} {}", t!("info-using-custom-css"), css_path.display());
                content
            }
            Err(e) => {
                eprintln!(
                    "{} {}: {}",
                    t!("warning-failed-to-load-css"),
                    css_path.display(),
                    e
                );
                eprintln!("{}", t!("info-falling-back-to-default-css"));
                include_str!("../resources/css/default.css").to_string()
            }
        }
    } else {
        include_str!("../resources/css/default.css").to_string()
    };

    // 写入CSS文件
    std::fs::write(output_path, &css_content)?;

    Ok(css_content)
}

/// 渲染mimetype文件
pub fn render_mimetype(output_path: &PathBuf) -> Result<()> {
    std::fs::write(output_path, "application/epub+zip")?;

    Ok(())
}

/// 渲染container.xml文件
pub fn render_container(output_path: &PathBuf) -> Result<()> {
    let container_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#;

    std::fs::write(output_path, container_content)?;

    Ok(())
}
