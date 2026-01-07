use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_chinese_chapters() {
    // 创建测试内容
    let content = r#"序
这是序章内容

1章
这是第一章内容，没有"第"字

第2章
这是第二章内容，有"第"字

番外：某件小事
这是番外内容，应该被忽略连贯性检查

3章
这是第三章内容，没有"第"字，中间插入了番外

第4张
这是第四章内容，使用"张"作为标识

尾声
这是尾声内容"#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--lang")
       .arg("zh-CN")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("章节号是递增的，符合要求"));
}

#[test]
fn test_english_chapters() {
    // 创建测试内容
    let content = r#"Chapter 1
This is the first chapter in English format.

Chapter 2
This is the second chapter.

Chapter Three
This is the third chapter with written number.

Section 1
This is a section.

Section Two
This is another section with written number."#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--lang")
       .arg("en")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detected chapters:"));
}

#[test]
fn test_roman_chapters() {
    // 创建测试内容
    let content = r#"Chapter I
This is the first chapter in Roman numerals.

Chapter II
This is the second chapter.

Chapter III
This is the third chapter.

Section I
This is a section in Roman numerals.

Section II
This is another section."#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--lang")
       .arg("en")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detected chapters:"));
}

#[test]
fn test_custom_rules() {
    // 创建自定义规则文件
    let rules_content = r#"[chapter]
regex = [
    "^Episode ",
    "^Special "
]

# 章节序号提取规则
[chapter_number_extraction]
rules = [
    { pattern = "^Episode ([0-9]+):", capture_group = 1, number_type = "arabic" },
    { pattern = "^Special ([0-9]+):", capture_group = 1, number_type = "arabic" }
]

[paragraph]
merge_lines = false
trim_whitespace = true"#;
    let rules_file = NamedTempFile::new().unwrap();
    std::fs::write(rules_file.path(), rules_content).unwrap();
    
    // 创建测试内容
    let content = r#"Episode 1: The Beginning
This is the first episode in a custom format.

Episode 2: The Journey
This is the second episode.

Episode 3: The End
This is the third episode.

Special 1: Bonus Content
This is bonus content."#;
    let content_file = NamedTempFile::new().unwrap();
    std::fs::write(content_file.path(), content).unwrap();
 let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--lang")
       .arg("en")
       .arg("--rules")
       .arg(rules_file.path())
       .arg(content_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detected chapters:"));
}

#[test]
fn test_discontinuous_chapters() {
    // 创建测试内容
    let content = r#"第1章
这是第一章

第3章
这是第三章，跳过了第二章

第2章
这是第二章，顺序不正确

第4章
这是第四章"#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--lang")
       .arg("zh-CN")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("警告：章节号不是递增的"));
}

#[test]
fn test_advanced_chapters() {
    // 创建测试内容
    let content = r#"第1章
这是第一章

第一章
这是第一章的另一种写法

第1节
这是第一节

1节
这是第一节的另一种写法

第1卷
这是第一卷

1卷
这是第一卷的另一种写法"#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--lang")
       .arg("en")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detected chapters:"));
}

#[test]
fn test_empty_file() {
    // 测试空文件处理
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), "").unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--lang")
       .arg("en")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detected chapters:"));
}

#[test]
fn test_no_chapters_file() {
    // 测试没有章节的文件（整个文件作为一个章节）
    let content = r#"This is a file without chapters.
It should be treated as a single chapter."#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--lang")
       .arg("en")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detected chapters:"));
}

#[test]
fn test_invalid_rules_file() {
    // 测试无效的规则文件
    let content = "这不是一个有效的TOML文件";
    let rules_file = NamedTempFile::new().unwrap();
    std::fs::write(rules_file.path(), content).unwrap();
    
    // 创建测试内容
    let test_content = r#"第1章
这是第一章"#;
    let test_file = NamedTempFile::new().unwrap();
    std::fs::write(test_file.path(), test_content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--rules")
       .arg(rules_file.path())
       .arg(test_file.path());
    
    cmd.assert()
        .failure();
}

#[test]
fn test_invalid_regex_file() {
    // 测试无效的正则表达式
    let rules_content = r#"[chapter]
regex = [
    "^(invalid regex pattern"
]

[paragraph]
merge_lines = false
trim_whitespace = true"#;
    let rules_file = NamedTempFile::new().unwrap();
    std::fs::write(rules_file.path(), rules_content).unwrap();
    
    // 创建测试内容
    let test_content = r#"第1章
这是第一章"#;
    let test_file = NamedTempFile::new().unwrap();
    std::fs::write(test_file.path(), test_content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--dry-run")
       .arg("--rules")
       .arg(rules_file.path())
       .arg(test_file.path());
    
    cmd.assert()
        .failure();
}

#[test]
fn test_print_outline() {
    // 创建测试内容
    let content = r#"第1章
这是第一章

第2章
这是第二章

第3章
这是第三章"#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--print-outline")
       .arg("--lang")
       .arg("en")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Chapter outline:"));
}

#[test]
fn test_verbose_mode() {
    // 创建测试内容
    let content = r#"第1章
这是第一章

第2章
这是第二章"#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("-v")
       .arg("--dry-run")
       .arg("--lang")
       .arg("en")
       .arg(temp_file.path());
    
    cmd.assert()
        .success();
}

#[test]
fn test_explain_mode() {
    // 创建测试内容
    let content = r#"第1章
这是第一章

第2章
这是第二章"#;
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();
    
    let mut cmd = Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("--explain")
       .arg("--dry-run")
       .arg("--lang")
       .arg("zh-CN")
       .arg(temp_file.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("使用的章节正则表达式："));
}
