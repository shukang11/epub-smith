# EpubSmith

A predictable, explainable, and reusable TXT to EPUB CLI tool written in Rust.

**Core Philosophy**: Focus on doing one thing well - generating standardized, stable EPUB files. For other formats, use downstream professional tools with our recommended pipelines.

[![中文版本](https://img.shields.io/badge/README-%E4%B8%AD%E6%96%87%E7%89%88%E6%9C%AC-blue)](README_zh.md)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Command Structure](#command-structure)
- [Custom Styling](#custom-styling)
- [Debugging and Performance](#debugging-and-performance)
- [Testing](#testing)
- [Recommended Pipelines](#recommended-pipelines)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)

## Features

- **Predictable Results**: Convert plain text files to well-structured EPUB books with consistent output
- **Easy to Use**: Simple command-line interface with sensible defaults
- **Customizable**: Support for custom parsing rules and templates
- **Chapter Coherence Check**: Automatically verify chapter numbering consistency
- **Performance Optimized**: Fast parsing and generation with performance metrics available
- **Debugging Tools**: Dry run mode, chapter outline preview, and detailed performance analysis
- **Custom Styling**: Support for custom CSS styles
- **EPUB Validation**: Optional integration with epubcheck for validation
- **Multiple Input Files**: Merge multiple TXT files into a single EPUB book
- **STDIN Support**: Read input from standard input for pipeline workflows
- **Structure Snapshots**: Export and import chapter structure for reuse and manual adjustment
- **Input Validation**: Diagnose text structure issues with preview commands

## Installation

### Using the Install Script

The easiest way to install EpubSmith is using the provided installation script. This script will automatically:
- Detect the latest release version from GitHub
- Identify your operating system and architecture
- Download the appropriate binary package
- Install it to a suitable location
- Configure your PATH environment variable if needed

#### Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/shukang11/epub-smith/main/install.sh | bash
```

This will install the latest version of EpubSmith on your system.

#### Install a Specific Version

```bash
curl -fsSL https://raw.githubusercontent.com/shukang11/epub-smith/main/install.sh | bash -s -- --version v0.2.0
```

#### Download Only

If you only want to download the binary without installing it:

```bash
curl -fsSL https://raw.githubusercontent.com/shukang11/epub-smith/main/install.sh | bash -s -- --download-only
```

#### Check Installation

After installation, you can verify that EpubSmith is installed correctly by running:

```bash
epub-smith --version
# or use the short name
epbs --version
```

You should see the installed version number. To get help, run:

```bash
epub-smith --help
# or
epbs --help
```

### From Source

1. Ensure you have Rust and Cargo installed. You can install them via [rustup](https://rustup.rs/).
2. Clone the repository:
   ```bash
   git clone https://github.com/shukang11/epub-smith.git
   cd epub-smith
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The executable will be available at `target/release/epub-smith`



## Quick Start

### Basic Usage

Convert a text file to EPUB:

```bash
epub-smith convert my_book.txt
```

Or use the short name:

```bash
epbs convert my_book.txt
```

Both commands will generate `my_book.epub` in the current directory.

### Custom Output File

```bash
epub-smith convert my_book.txt -o my_custom_book.epub
```

### Specify Author and Title

```bash
epub-smith convert my_book.txt --author "John Doe" --title "My Book"
```

### Preview Chapters (Dry Run)

Preview the chapter structure without generating EPUB:

```bash
epub-smith preview dry-run my_book.txt
```

Or with language setting:

```bash
epub-smith --lang zh-CN preview dry-run my_book.txt
```

### Print Chapter Outline

```bash
epub-smith preview outline my_book.txt
```

### Multiple Input Files

Merge multiple TXT files into a single EPUB book:

```bash
# Merge all TXT files in src directory
epub-smith convert src/*.txt -o combined_book.epub

# Merge specific files in order
epub-smith convert chapter1.txt chapter2.txt chapter3.txt -o book.epub
```

### STDIN Support

Read input from standard input for pipeline workflows:

```bash
# Pipe content directly to EpubSmith
cat novel.txt | epub-smith convert - -o novel.epub

# Use with other CLI tools
grep -v "#" raw.txt | sed 's/\r//g' | epub-smith convert - -o cleaned.epub
```

### Structure Snapshots

Export chapter structure to a JSON file for reuse and manual adjustment. Snapshots contain complete book information including metadata and chapter contents.

#### Generate Snapshot
```bash
# Export structure snapshot from TXT file
epub-smith snapshot save novel.txt -o structure.json
```

#### Use Snapshot
```bash
# Use existing structure snapshot to generate EPUB (NO TXT file required)
epub-smith snapshot load structure.json -o custom_book.epub

# Manually edit structure.json to adjust chapters, then re-use
```

### Custom Rules

Use a custom rules file to control how EpubSmith parses your text:

```bash
epub-smith convert my_book.txt -r custom_rules.toml
```

Example rules file for anthology format (`01【Title】`):

```toml
[chapter]
regex = [
    "^[\t\\s]*[0-9]+【",
    "^[\t\\s]*[０-９]+【",
    "^[\t\\s]*[ⅠⅡⅢⅣⅤⅥⅦⅧⅨⅩ]+【"
]

[chapter_number_extraction]
rules = [
    { pattern = "^[\t\\s]*([0-9]+)【", capture_group = 1, number_type = "arabic" }
]

[paragraph]
merge_lines = false
trim_whitespace = true
```

## Command Structure

EpubSmith uses a subcommand-based CLI structure:

```
epub-smith [GLOBAL OPTIONS] <SUBCOMMAND> [SUBCOMMAND OPTIONS]
```

### Global Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Show detailed logs |
| `--debug` | Enable performance analysis |
| `--lang <LANGUAGE>` | Specify output language (e.g., en, zh-CN) |

### Subcommands

| Subcommand | Description |
|------------|-------------|
| `convert` | Convert TXT files to EPUB (default command) |
| `template export <DIRECTORY>` | Export style templates to a directory |
| `snapshot save <INPUT> -o <FILE>` | Save chapter structure to a snapshot file |
| `snapshot load <FILE> -o <OUTPUT>` | Generate EPUB from a snapshot file |
| `preview outline <INPUT>` | Print chapter outline |
| `preview dry-run <INPUT>` | Show detailed chapter structure without generating EPUB |

### Convert Options

| Option | Description |
|--------|-------------|
| `<INPUT>` | Input TXT file(s) or directory (use `-` for STDIN) |
| `-r, --rules <FILE>` | Rules file path |
| `-o, --output <FILE>` | Output EPUB file path [default: book.epub] |
| `-e, --encoding <ENCODING>` | Force input file encoding |
| `--title <TITLE>` | Specify book title |
| `--author <AUTHOR>` | Specify book author |
| `--cover <FILE>` | Specify cover image path |
| `--language <LANGUAGE>` | Specify book language [default: zh-CN] |
| `--check` | Validate EPUB with epubcheck |
| `--style <FILE>` | Specify custom CSS style file |

## Custom Styling

EpubSmith uses Tera templates and CSS for styling. You can export the default templates and modify them:

```bash
epub-smith template export my_templates
```

This will create a directory with the default templates and CSS files. You can then modify these files and use them with your custom CSS:

```bash
epub-smith convert input.txt --style my_templates/default.css
```


## Debugging and Performance

### Dry Run

The `preview-dry-run` subcommand allows you to preview how EpubSmith will parse your file without generating an EPUB:

```bash
epub-smith preview dry-run input.txt
```

### Verbose Logging

Use `--verbose` to see detailed logs during the conversion process:

```bash
epub-smith --verbose convert input.txt
```

Or with the short name:

```bash
epbs --verbose convert input.txt
```

### Performance Analysis

The `--debug` option enables performance metrics, showing the time taken for each phase of the conversion:

```bash
epub-smith --debug convert input.txt
```

## Testing

Run the test suite to ensure everything is working correctly:

```bash
cargo test
```

## Recommended Pipelines

EpubSmith focuses on generating standardized, stable EPUB files. For other formats, we recommend using professional downstream tools. Here are some common pipelines:

### EPUB → Kindle (MOBI/KFX)

```bash
# Step 1: Generate EPUB with EpubSmith
epub-smith convert novel.txt -o novel.epub

# Step 2: Convert to MOBI using Calibre's ebook-convert
ebook-convert novel.epub novel.mobi

# Step 3: Send to Kindle via email or Calibre
```

### EPUB → PDF

```bash
# Step 1: Generate EPUB with EpubSmith
epub-smith convert novel.txt -o novel.epub

# Step 2: Convert to PDF using Calibre's ebook-convert
ebook-convert novel.epub novel.pdf

# Or use pandoc for more customization
pandoc novel.epub -o novel.pdf --pdf-engine=xelatex
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This project uses the [Tera](https://tera.netlify.app/) templating engine
- EPUB generation is powered by the [zip](https://crates.io/crates/zip) crate
- Command-line interface is built with [clap](https://clap.rs/)

## Contact

For questions, suggestions, or issues, please open an [issue](https://github.com/shukang11/epub-smith/issues) on GitHub.
