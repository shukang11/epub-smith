use std::path::PathBuf;
use tempfile::{tempdir, NamedTempFile};
use booksmith::{packager, models::{Book, Meta, Rules}, config::Config};

#[test]
fn test_add_mimetype() {
    // 创建临时文件用于测试
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    
    // 创建ZipWriter
    let file = std::fs::File::create(&file_path).unwrap();
    let mut zip = zip::write::ZipWriter::new(file);
    
    // 测试添加mimetype
    let result = packager::add_mimetype(&mut zip);
    assert!(result.is_ok());
    
    // 完成ZIP写入
    let result = zip.finish();
    assert!(result.is_ok());
    
    // 验证文件存在且大小合理
    let metadata = std::fs::metadata(&file_path).unwrap();
    assert!(metadata.len() > 0);
}

#[test]
fn test_add_file_to_zip() {
    // 创建临时目录和文件
    let temp_dir = tempdir().unwrap();
    let temp_file_path = temp_dir.path().join("test.txt");
    
    // 写入测试内容
    std::fs::write(&temp_file_path, "Test content").unwrap();
    
    // 创建ZIP文件
    let zip_file = NamedTempFile::new().unwrap();
    let zip_path = zip_file.path().to_path_buf();
    
    // 创建ZipWriter
    let file = std::fs::File::create(&zip_path).unwrap();
    let mut zip = zip::write::ZipWriter::new(file);
    
    // 测试添加文件到ZIP
    let result = packager::add_file_to_zip(&mut zip, &temp_file_path, "test.txt");
    assert!(result.is_ok());
    
    // 完成ZIP写入
    let result = zip.finish();
    assert!(result.is_ok());
    
    // 验证ZIP文件大小
    let metadata = std::fs::metadata(&zip_path).unwrap();
    assert!(metadata.len() > 0);
}

#[test]
fn test_validate_epub() {
    // 创建临时文件
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    
    // 写入足够大小的内容
    std::fs::write(&file_path, "x".repeat(200)).unwrap();
    
    // 测试验证EPUB
    let result = packager::validate_epub(&file_path);
    assert!(result.is_ok());
}

#[test]
fn test_validate_epub_too_small() {
    // 创建临时文件
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    
    // 写入过小的内容
    std::fs::write(&file_path, "too small").unwrap();
    
    // 测试验证EPUB（应该失败）
    let result = packager::validate_epub(&file_path);
    assert!(result.is_err());
}

#[test]
fn test_package_epub() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    
    // 创建测试HTML文件
    let chapter1_path = temp_dir.path().join("chapter1.xhtml");
    std::fs::write(&chapter1_path, "<html><body><h1>Chapter 1</h1></body></html>").unwrap();
    
    let chapter2_path = temp_dir.path().join("chapter2.xhtml");
    std::fs::write(&chapter2_path, "<html><body><h1>Chapter 2</h1></body></html>").unwrap();
    
    let container_path = temp_dir.path().join("container.xml");
    std::fs::write(&container_path, "<container></container>").unwrap();
    
    // 准备HTML文件列表
    let xhtml_files = vec![
        chapter1_path.clone(),
        chapter2_path.clone(),
        container_path.clone()
    ];
    
    // 创建输出路径
    let output_path = temp_dir.path().join("test.epub");
    
    // 创建配置
    let config = Config {
        rules: Rules::default(),
        output: output_path.clone(),
        check: false,
        explain: false,
        encoding: None
    };
    
    // 创建空的Book结构体
    let book = Book {
        meta: Meta {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            language: "zh-CN".to_string(),
            identifier: "test-identifier".to_string(),
            modified: "2026-01-01T00:00:00+00:00".to_string(),
            cover: None
        },
        chapters: Vec::new()
    };
    
    // 测试打包EPUB
    let result = packager::package_epub(&book, &xhtml_files, &config);
    assert!(result.is_ok());
    
    // 验证EPUB文件存在
    assert!(output_path.exists());
    
    // 验证EPUB文件大小
    let metadata = std::fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0);
}

#[test]
fn test_package_epub_with_check() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    
    // 创建测试HTML文件
    let chapter1_path = temp_dir.path().join("chapter1.xhtml");
    std::fs::write(&chapter1_path, "<html><body><h1>Chapter 1</h1></body></html>").unwrap();
    
    // 准备HTML文件列表
    let xhtml_files = vec![chapter1_path.clone()];
    
    // 创建输出路径
    let output_path = temp_dir.path().join("test_with_check.epub");
    
    // 创建配置（带check选项）
    let config = Config {
        rules: Rules::default(),
        output: output_path.clone(),
        check: true,
        explain: false,
        encoding: None
    };
    
    // 创建空的Book结构体
    let book = Book {
        meta: Meta {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            language: "zh-CN".to_string(),
            identifier: "test-identifier".to_string(),
            modified: "2026-01-01T00:00:00+00:00".to_string(),
            cover: None
        },
        chapters: Vec::new()
    };
    
    // 测试打包EPUB并验证
    let result = packager::package_epub(&book, &xhtml_files, &config);
    assert!(result.is_ok());
    
    // 验证EPUB文件存在
    assert!(output_path.exists());
}
