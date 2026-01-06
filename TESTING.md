# BookSmith 测试说明

## 测试用例概述

BookSmith 包含多种测试用例，用于验证不同章节格式和功能的正确性。

## 测试文件目录

所有测试文件都位于 `examples/` 目录下：

### 中文章节测试
- `test_new_features.txt` - 测试中文章节格式，包含有"第"和无"第"的章节，以及番外等特殊章节
- `chapter_variations.txt` - 测试各种中文章节格式变体
- `discontinuous_chapters.txt` - 测试不连续章节的连贯性检查
- `advanced_chapters.txt` - 测试复杂中文章节格式

### 英文章节测试
- `english_chapters.txt` - 测试英文章节格式，包含数字、英文单词和罗马数字

### 罗马数字章节测试
- `roman_chapters.txt` - 测试罗马数字章节格式

### 自定义规则测试
- `custom_rule_chapters.txt` - 测试自定义章节格式（Episode X: Title）
- `custom_rules.toml` - 自定义规则配置文件

### 边界情况测试
- `empty.txt` - 测试空文件处理
- `special_chars.txt` - 测试特殊字符处理

## 运行测试

### 运行所有测试

```bash
cargo test
```

### 运行集成测试

```bash
cargo test --test integration_test
```

### 运行单元测试

```bash
cargo test --test unit_test
```

### 测试单个章节格式

```bash
cargo run -- --dry-run examples/test_new_features.txt  # 测试中文章节
cargo run -- --dry-run examples/english_chapters.txt  # 测试英文章节
cargo run -- --dry-run examples/roman_chapters.txt  # 测试罗马数字章节
cargo run -- --dry-run --rules examples/custom_rules.toml examples/custom_rule_chapters.txt  # 测试自定义规则
```

## 测试说明

### 中文章节测试
测试中文章节的检测和连贯性检查，支持：
- 第X章、X章、第一章等格式
- 中文数字和阿拉伯数字
- 多种章节标识（章、节、卷、张）
- 特殊章节（番外、序、尾声等）

### 英文章节测试
测试英文章节的检测和连贯性检查，支持：
- Chapter X 格式
- Chapter 英文单词格式（如 Chapter Three）
- Section X 格式

### 罗马数字章节测试
测试罗马数字章节的检测和连贯性检查，支持：
- Chapter I, II, III 格式
- Section I, II, III 格式

### 自定义规则测试
测试自定义规则的章节检测和连贯性检查，支持：
- 自定义章节格式
- 自定义序号提取规则
- 优先级匹配

## 测试结果解读

### 成功情况
- 显示章节号是递增的，符合要求
- 正确检测所有章节
- 没有警告信息

### 警告情况
- 发现重复的章节号：表示有相同的章节号出现
- 章节号不是递增的：表示章节号顺序有问题
- 未检测到明确的章节号：表示无法提取章节号，跳过连贯性检查

## 扩展测试

你可以创建自己的测试文件和规则文件来测试更多章节格式：

1. 创建新的测试文本文件
2. 创建对应的规则文件（可选）
3. 运行测试命令
4. 查看测试结果

## 注意事项

- 测试文件必须使用 UTF-8 编码
- 规则文件必须符合 TOML 格式
- 正则表达式必须符合 Rust 正则表达式语法
