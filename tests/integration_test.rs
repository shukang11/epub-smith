#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;
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
    cmd.arg("--lang")
        .arg("zh-CN")
        .arg("preview")
        .arg("dry-run")
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
    cmd.arg("--lang")
        .arg("en")
        .arg("preview")
        .arg("dry-run")
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
    cmd.arg("--lang")
        .arg("en")
        .arg("preview")
        .arg("dry-run")
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
    cmd.arg("--lang")
        .arg("en")
        .arg("preview")
        .arg("dry-run")
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
    cmd.arg("--lang")
        .arg("zh-CN")
        .arg("preview")
        .arg("dry-run")
        .arg(temp_file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("警告：章节号顺序异常"));
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
    cmd.arg("--lang")
        .arg("en")
        .arg("preview")
        .arg("dry-run")
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
    cmd.arg("--lang")
        .arg("en")
        .arg("preview")
        .arg("dry-run")
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
    cmd.arg("--lang")
        .arg("en")
        .arg("preview")
        .arg("dry-run")
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
    cmd.arg("preview")
        .arg("dry-run")
        .arg("--rules")
        .arg(rules_file.path())
        .arg(test_file.path());

    cmd.assert().failure();
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
    cmd.arg("preview-dry-run")
        .arg("--rules")
        .arg(rules_file.path())
        .arg(test_file.path());

    cmd.assert().failure();
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
    cmd.arg("--lang")
        .arg("en")
        .arg("preview")
        .arg("outline")
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
    cmd.arg("--verbose")
        .arg("--lang")
        .arg("en")
        .arg("preview")
        .arg("dry-run")
        .arg(temp_file.path());

    cmd.assert().success();
}

#[test]
fn test_snapshot_export() {
    // 创建测试内容
    let content = r#"第1章
这是第一章

第2章
这是第二章

第3章
这是第三章"#;
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();

    // 创建临时快照文件路径
    let snapshot_path = tempfile::NamedTempFile::new().unwrap();
    let snapshot_path_str = snapshot_path.path().to_str().unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("snapshot")
        .arg("save")
        .arg(temp_file.path())
        .arg("-o")
        .arg(snapshot_path_str);

    cmd.assert().success();

    // 验证快照文件是否存在且不为空
    assert!(std::fs::metadata(snapshot_path_str).unwrap().len() > 0);

    // 验证快照文件是否为有效的JSON
    let snapshot_content = std::fs::read_to_string(snapshot_path_str).unwrap();
    let _: serde_json::Value = serde_json::from_str(&snapshot_content).unwrap();
}

#[test]
fn test_lint_snapshot_export() {
    // 创建测试内容
    let content = r#"第1章
这是第一章

第2章
这是第二章

第3章
这是第三章"#;
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();

    // 创建临时快照文件路径
    let snapshot_path = tempfile::NamedTempFile::new().unwrap();
    let snapshot_path_str = snapshot_path.path().to_str().unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("snapshot")
        .arg("save")
        .arg(temp_file.path())
        .arg("-o")
        .arg(snapshot_path_str);

    cmd.assert().success();

    // 验证快照文件是否存在且不为空
    assert!(std::fs::metadata(snapshot_path_str).unwrap().len() > 0);
}

#[test]
fn test_snapshot_import() {
    // 首先创建一个快照文件
    let content = r#"第1章
这是第一章

第2章
这是第二章

第3章
这是第三章"#;
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();

    // 创建临时快照文件路径
    let snapshot_path = tempfile::NamedTempFile::new().unwrap();
    let snapshot_path_str = snapshot_path.path().to_str().unwrap();

    // 导出快照
    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("snapshot")
        .arg("save")
        .arg(temp_file.path())
        .arg("-o")
        .arg(snapshot_path_str);
    cmd.assert().success();

    // 创建临时输出文件路径
    let output_path = tempfile::NamedTempFile::new().unwrap();
    let output_path_str = output_path.path().to_str().unwrap();

    // 使用快照生成EPUB
    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("snapshot")
        .arg("load")
        .arg(snapshot_path_str)
        .arg("-o")
        .arg(output_path_str);

    cmd.assert().success();

    // 验证EPUB文件是否生成
    assert!(std::fs::metadata(output_path_str).unwrap().len() > 0);
}

#[test]
fn test_snapshot_with_input_files_error() {
    // 创建测试内容
    let content = r#"第1章
这是第一章"#;
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();

    // 创建临时快照文件路径
    let snapshot_path = tempfile::NamedTempFile::new().unwrap();
    let snapshot_path_str = snapshot_path.path().to_str().unwrap();

    // 导出快照
    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("snapshot")
        .arg("save")
        .arg(temp_file.path())
        .arg("-o")
        .arg(snapshot_path_str);
    cmd.assert().success();

    // snapshot-load 命令不支持同时指定输入文件，会报错
    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("snapshot")
        .arg("load")
        .arg(snapshot_path_str)
        .arg(temp_file.path());

    cmd.assert().failure();
}

// 移除 explain 模式测试，因为该选项在新结构中被移除
// #[test]
// fn test_explain_mode() {
//     // 创建测试内容
//     let content = r#"第1章
// 这是第一章
//
// 第2章
// 这是第二章"#;
//     let temp_file = NamedTempFile::new().unwrap();
//     std::fs::write(temp_file.path(), content).unwrap();
//
//     let mut cmd = Command::cargo_bin("epub-smith").unwrap();
//     cmd.arg("--explain")
//         .arg("--dry-run")
//         .arg("--lang")
//         .arg("zh-CN")
//         .arg(temp_file.path());
//
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("使用的章节正则表达式："));
// }

#[test]
fn test_auto_cover_generates_cover_png() {
    // 创建测试内容
    let content = r#"第1章
这是第一章

第2章
这是第二章"#;
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();

    let output_path = temp_file.path().with_extension("epub");
    let output_str = output_path.to_str().unwrap();

    // --cover auto 生成默认封面
    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("convert")
        .arg(temp_file.path())
        .arg("--cover")
        .arg("auto")
        .arg("-o")
        .arg(output_str);
    cmd.assert().success();

    // 验证 EPUB 是合法 zip 且包含封面图片与封面页
    let file = std::fs::File::open(output_str).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let names: Vec<String> = archive.file_names().map(|s| s.to_string()).collect();
    assert!(
        names.contains(&"cover.png".to_string()),
        "EPUB 应包含 cover.png，实际: {names:?}"
    );
    assert!(
        names.contains(&"cover.xhtml".to_string()),
        "EPUB 应包含封面页 cover.xhtml，实际: {names:?}"
    );

    // cover.png 必须是合法 PNG
    {
        let mut cover_png = archive.by_name("cover.png").unwrap();
        let mut head = [0u8; 8];
        std::io::Read::read_exact(&mut cover_png, &mut head).unwrap();
        assert_eq!(&head, b"\x89PNG\r\n\x1a\n", "cover.png 不是合法 PNG");
    }

    // OPF 必须声明封面图片与封面页
    let mut opf = archive.by_name("content.opf").unwrap();
    let mut opf_content = String::new();
    std::io::Read::read_to_string(&mut opf, &mut opf_content).unwrap();
    assert!(
        opf_content.contains("properties=\"cover-image\""),
        "OPF 缺少 cover-image 声明"
    );
    assert!(
        opf_content.contains("id=\"cover-image\""),
        "OPF 缺少封面 item id"
    );
    assert!(
        opf_content.contains("image/png"),
        "OPF 封面 media-type 应为 image/png"
    );
    assert!(
        opf_content.contains("name=\"cover\" content=\"cover-image\""),
        "OPF metadata 应声明封面关联"
    );
    assert!(
        opf_content.contains("href=\"cover.xhtml\""),
        "OPF manifest 应包含封面页"
    );

    // spine 首项引用封面页（manifest 第一项 = cover.xhtml = item-1）
    let spine_start = opf_content.find("<spine>").unwrap();
    let spine_end = opf_content.find("</spine>").unwrap();
    let spine = &opf_content[spine_start..spine_end];
    assert!(
        spine.contains("itemref idref=\"item-1\""),
        "spine 首项应为封面页，实际 spine: {spine}"
    );

    std::fs::remove_file(output_str).ok();
}

#[test]
fn test_auto_cover_snapshot_load() {
    // 创建测试内容并保存快照
    let content = r#"第1章
这是第一章

第2章
这是第二章"#;
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), content).unwrap();

    let snapshot_path = tempfile::NamedTempFile::new().unwrap();
    let snapshot_str = snapshot_path.path().to_str().unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("snapshot")
        .arg("save")
        .arg(temp_file.path())
        .arg("-o")
        .arg(snapshot_str);
    cmd.assert().success();

    // snapshot load --cover auto 也应生成封面
    let output_path = temp_file.path().with_extension("snap_epub");
    let output_str = output_path.to_str().unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("snapshot")
        .arg("load")
        .arg(snapshot_str)
        .arg("--cover")
        .arg("auto")
        .arg("-o")
        .arg(output_str);
    cmd.assert().success();

    let file = std::fs::File::open(output_str).unwrap();
    let archive = zip::ZipArchive::new(file).unwrap();
    let names: Vec<String> = archive.file_names().map(|s| s.to_string()).collect();
    assert!(
        names.contains(&"cover.png".to_string()),
        "snapshot load --cover auto 应生成封面，实际: {names:?}"
    );

    std::fs::remove_file(output_str).ok();
}

#[test]
fn test_template_export_includes_cover_svg() {
    let export_dir = tempfile::tempdir().unwrap();
    let export_str = export_dir.path().to_str().unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("epub-smith").unwrap();
    cmd.arg("template").arg("export").arg(export_str);
    cmd.assert().success();

    let cover_svg_path = export_dir.path().join("cover.svg");
    assert!(cover_svg_path.exists(), "template export 应包含 cover.svg");
    let content = std::fs::read_to_string(&cover_svg_path).unwrap();
    assert!(
        content.contains("{{ title_lines }}") && content.contains("{{ author }}"),
        "cover.svg 应包含占位符"
    );

    // 预览页应包含章节视图、目录、示例封面与翻页交互
    let preview_path = export_dir.path().join("preview.html");
    assert!(preview_path.exists(), "template export 应包含 preview.html");
    let preview = std::fs::read_to_string(&preview_path).unwrap();
    assert!(
        preview.contains("class=\"chapter-view"),
        "preview.html 应包含章节视图"
    );
    assert!(preview.contains("data-idx"), "preview.html 应包含目录链接");
    assert!(
        preview.contains("preview-cover"),
        "preview.html 应包含示例封面"
    );
    assert!(
        preview.contains("nextChapter") && preview.contains("prevChapter"),
        "preview.html 应包含翻页交互"
    );
}
