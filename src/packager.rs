use std::io::Write;
use std::path::PathBuf;
use anyhow::{Context, Result};
use zip::write::{ZipWriter, FileOptions};
use zip::CompressionMethod;

use crate::{models::Book, config::Config};

/// 将书籍打包成EPUB文件
pub fn package_epub(_book: &Book, xhtml_files: &[PathBuf], config: &Config) -> Result<()> {
    // 创建EPUB文件
    let file = std::fs::File::create(&config.output)?;
    
    // 初始化ZipWriter
    let mut zip = ZipWriter::new(file);
    
    // 首先添加mimetype文件（必须无压缩）
    add_mimetype(&mut zip)?;
    
    // 然后添加其他文件
    for file_path in xhtml_files {
        // 获取文件名
        let filename = file_path.file_name()
            .and_then(|os_str| os_str.to_str())
            .context(format!("Invalid file path: {}", file_path.display()))?;
        
        // 如果是mimetype文件，跳过（已经添加过）
        if filename == "mimetype" {
            continue;
        }
        
        // 如果是container.xml文件，确保它在META-INF目录下
        let entry_name = if filename == "container.xml" {
            "META-INF/container.xml".to_string()
        } else {
            filename.to_string()
        };
        
        // 添加文件到ZIP
        add_file_to_zip(&mut zip, file_path, &entry_name)?;
    }
    
    // 完成ZIP写入
    zip.finish()?;
    
    // 如果需要验证EPUB，调用epubcheck
    if config.check {
        validate_epub(&config.output)?;
    }
    
    Ok(())
}

/// 添加mimetype文件到ZIP（必须无压缩）
fn add_mimetype(zip: &mut ZipWriter<std::fs::File>) -> Result<()> {
    // 创建mimetype文件条目，无压缩
    zip.start_file("mimetype", FileOptions::default().compression_method(CompressionMethod::Stored))?;
    
    // 写入mimetype内容
    zip.write_all(b"application/epub+zip")?;
    
    Ok(())
}

/// 添加文件到ZIP
fn add_file_to_zip(
    zip: &mut ZipWriter<std::fs::File>, 
    file_path: &PathBuf, 
    entry_name: &str
) -> Result<()> {
    // 读取文件内容
    let content = std::fs::read(file_path)?;
    
    // 创建ZIP条目，使用deflate压缩
    zip.start_file(entry_name, FileOptions::default().compression_method(CompressionMethod::Deflated))?;
    
    // 写入文件内容
    zip.write_all(&content)?;
    
    Ok(())
}

/// 验证EPUB文件
fn validate_epub(path: &PathBuf) -> Result<()> {
    // 注意：这里应该调用epubcheck工具
    // 由于epubcheck是一个外部工具，我们暂时只进行简单的验证
    // 检查文件是否存在且大小合理
    
    let metadata = std::fs::metadata(path)?;
    
    if metadata.len() < 100 {
        anyhow::bail!("EPUB file is too small, may be invalid: {}", path.display());
    }
    
    println!("EPUB file created successfully: {}", path.display());
    println!("Note: epubcheck validation is not implemented yet.");
    println!("To validate your EPUB, download epubcheck from https://github.com/w3c/epubcheck");
    println!("and run: java -jar epubcheck.jar {}", path.display());
    
    Ok(())
}
