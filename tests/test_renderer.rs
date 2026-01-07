use epub_smith::{
    config::Config,
    models::{Book, Chapter, Meta, Rules},
    renderer,
};
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn test_render_mimetype() {
    // 创建临时文件
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();

    // 测试渲染mimetype
    let result = renderer::render_mimetype(&file_path);
    assert!(result.is_ok());

    // 验证文件内容
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "application/epub+zip");
}

#[test]
fn test_render_container() {
    // 创建临时文件
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();

    // 测试渲染container.xml
    let result = renderer::render_container(&file_path);
    assert!(result.is_ok());

    // 验证文件内容包含必要的XML结构
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("container"));
    assert!(content.contains("rootfiles"));
    assert!(content.contains("content.opf"));
}

#[test]
fn test_render_css() {
    // 创建临时文件
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();

    // 测试渲染CSS（使用默认CSS）
    let result = renderer::render_css(&file_path, &None);
    assert!(result.is_ok());

    // 验证文件内容
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("use std::path::PathBuf"));
}

#[test]
fn test_initialize_tera() {
    // 测试初始化Tera模板引擎
    let result = renderer::initialize_tera();
    assert!(result.is_ok());

    let tera = result.unwrap();
    // 验证模板已加载
    assert!(tera.get_template("chapter.xhtml").is_ok());
    assert!(tera.get_template("nav.xhtml").is_ok());
    assert!(tera.get_template("content.opf").is_ok());
}

#[test]
fn test_render_book() {
    // 创建测试书籍
    let book = Book {
        meta: Meta {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            language: "zh-CN".to_string(),
            identifier: "test-identifier".to_string(),
            modified: "2026-01-01T00:00:00+00:00".to_string(),
            cover: None,
        },
        chapters: vec![
            Chapter {
                title: "Chapter 1".to_string(),
                paragraphs: vec!["Content of chapter 1".to_string()],
                slug: "chapter-1".to_string(),
                start_line: 0,
                end_line: 10,
            },
            Chapter {
                title: "Chapter 2".to_string(),
                paragraphs: vec!["Content of chapter 2".to_string()],
                slug: "chapter-2".to_string(),
                start_line: 11,
                end_line: 20,
            },
        ],
    };

    // 创建配置
    let config = Config {
        rules: Rules::default(),
        output: PathBuf::from("test.epub"),
        check: false,
        explain: false,
        encoding: None,
        style: None,
    };

    // 测试渲染书籍
    let result = renderer::render_book(&book, &config);
    assert!(result.is_ok());

    let xhtml_files = result.unwrap();

    // 验证生成了足够的文件
    assert!(xhtml_files.len() >= 6); // 2 chapters + nav + css + opf + mimetype + container

    // 验证所有文件都存在
    for file_path in &xhtml_files {
        assert!(file_path.exists());
        assert!(file_path.is_file());
    }

    // 验证文件类型
    let mut has_chapter_file = false;
    let mut has_nav_file = false;
    let mut has_css_file = false;
    let mut has_opf_file = false;
    let mut has_mimetype_file = false;
    let mut has_container_file = false;

    for file_path in &xhtml_files {
        let filename = file_path.file_name().unwrap().to_string_lossy().to_string();
        if filename.starts_with("chapter_") && filename.ends_with(".xhtml") {
            has_chapter_file = true;
        } else if filename == "nav.xhtml" {
            has_nav_file = true;
        } else if filename == "style.css" {
            has_css_file = true;
        } else if filename == "content.opf" {
            has_opf_file = true;
        } else if filename == "mimetype" {
            has_mimetype_file = true;
        } else if filename == "container.xml" {
            has_container_file = true;
        }
    }

    assert!(has_chapter_file);
    assert!(has_nav_file);
    assert!(has_css_file);
    assert!(has_opf_file);
    assert!(has_mimetype_file);
    assert!(has_container_file);
}

#[test]
fn test_render_chapter() {
    // 创建测试章节
    let chapter = Chapter {
        title: "Test Chapter".to_string(),
        paragraphs: vec!["Test content".to_string()],
        slug: "test-chapter".to_string(),
        start_line: 0,
        end_line: 10,
    };

    // 创建测试书籍
    let book = Book {
        meta: Meta {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            language: "zh-CN".to_string(),
            identifier: "test-identifier".to_string(),
            modified: "2026-01-01T00:00:00+00:00".to_string(),
            cover: None,
        },
        chapters: vec![chapter.clone()],
    };

    // 初始化Tera
    let tera = renderer::initialize_tera().unwrap();

    // 创建临时文件
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();

    // 测试渲染章节
    let result = renderer::render_chapter(&chapter, 1, &book, &tera, &file_path);
    assert!(result.is_ok());

    // 验证文件内容
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Test Chapter"));
    assert!(content.contains("Test content"));
    assert!(content.contains("xhtml"));
}

#[test]
fn test_render_nav() {
    // 创建测试书籍
    let book = Book {
        meta: Meta {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            language: "zh-CN".to_string(),
            identifier: "test-identifier".to_string(),
            modified: "2026-01-01T00:00:00+00:00".to_string(),
            cover: None,
        },
        chapters: vec![
            Chapter {
                title: "Chapter 1".to_string(),
                paragraphs: vec!["Content 1".to_string()],
                slug: "chapter-1".to_string(),
                start_line: 0,
                end_line: 10,
            },
            Chapter {
                title: "Chapter 2".to_string(),
                paragraphs: vec!["Content 2".to_string()],
                slug: "chapter-2".to_string(),
                start_line: 11,
                end_line: 20,
            },
        ],
    };

    // 初始化Tera
    let tera = renderer::initialize_tera().unwrap();

    // 创建临时文件
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();

    // 测试渲染导航
    let result = renderer::render_nav(&book, &tera, &file_path);
    assert!(result.is_ok());

    // 验证文件内容
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Test Book"));
    assert!(content.contains("Chapter 1"));
    assert!(content.contains("Chapter 2"));
    assert!(content.contains("nav"));
}

#[test]
fn test_render_opf() {
    // 创建测试书籍
    let book = Book {
        meta: Meta {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            language: "zh-CN".to_string(),
            identifier: "test-identifier".to_string(),
            modified: "2026-01-01T00:00:00+00:00".to_string(),
            cover: None,
        },
        chapters: vec![Chapter {
            title: "Chapter 1".to_string(),
            paragraphs: vec!["Content 1".to_string()],
            slug: "chapter-1".to_string(),
            start_line: 0,
            end_line: 10,
        }],
    };

    // 初始化Tera
    let tera = renderer::initialize_tera().unwrap();

    // 创建临时目录和测试XHTML文件
    let temp_dir = tempfile::tempdir().unwrap();
    let chapter_path = temp_dir.path().join("chapter_001.xhtml");
    std::fs::write(
        &chapter_path,
        "<html><body><h1>Chapter 1</h1></body></html>",
    )
    .unwrap();

    let xhtml_files = vec![chapter_path.clone()];

    // 创建临时OPF文件
    let opf_path = temp_dir.path().join("content.opf");

    // 测试渲染OPF
    let result = renderer::render_opf(&book, &xhtml_files, &tera, &opf_path);
    assert!(result.is_ok());

    // 验证文件内容
    let content = std::fs::read_to_string(&opf_path).unwrap();
    assert!(content.contains("Test Book"));
    assert!(content.contains("chapter_001.xhtml"));
    assert!(content.contains("opf"));
}
