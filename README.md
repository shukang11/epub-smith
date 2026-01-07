# EpubSmith

A predictable, explainable, and reusable TXT to EPUB CLI tool written in Rust.

**Core Philosophy**: Focus on doing one thing well - generating standardized, stable EPUB files. For other formats, use downstream professional tools with our recommended pipelines.

## Features

- **Predictable Results**: Convert plain text files to well-structured EPUB books with consistent output
- **Easy to Use**: Simple command-line interface with sensible defaults
- **Customizable**: Support for custom parsing rules and templates
- **Multi-language Support**: Built-in internationalization for English, Chinese, and other languages
- **Chapter Coherence Check**: Automatically verify chapter numbering consistency
- **Performance Optimized**: Fast parsing and generation with performance metrics available
- **Debugging Tools**: Dry run mode, chapter outline preview, and detailed performance analysis
- **Custom Styling**: Support for custom CSS styles
- **EPUB Validation**: Optional integration with epubcheck for validation
- **Multiple Input Files**: Merge multiple TXT files into a single EPUB book
- **STDIN Support**: Read input from standard input for pipeline workflows
- **Structure Snapshots**: Export and import chapter structure for reuse and manual adjustment
- **Input Validation**: Diagnose text structure issues with lint mode

## Installation

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

### Install via Cargo

```bash
cargo install epub-smith
```

## Quick Start

### Basic Usage

Convert a text file to EPUB:

```bash
epub-smith my_book.txt
```

Or use the short name:

```bash
epbs my_book.txt
```

Both commands will generate `book.epub` in the current directory.

### Custom Output File

```bash
epub-smith --output my_custom_book.epub my_book.txt
```

Or with the short name:

```bash
epbs --output my_custom_book.epub my_book.txt
```

### Specify Author and Title

```bash
epub-smith --author "John Doe" --title "My Book" my_book.txt
```

Or with the short name:

```bash
epbs --author "John Doe" --title "My Book" my_book.txt
```

### Dry Run (Preview Chapters)

```bash
epub-smith --dry-run my_book.txt
```

Or with the short name:

```bash
epbs --dry-run my_book.txt
```

### Print Chapter Outline

```bash
epub-smith --print-outline my_book.txt
```

Or with the short name:

```bash
epbs --print-outline my_book.txt
```

### Multiple Input Files

Merge multiple TXT files into a single EPUB book:

```bash
# Merge all TXT files in src directory
epub-smith src/*.txt -o combined_book.epub

# Merge specific files in order
epub-smith chapter1.txt chapter2.txt chapter3.txt -o book.epub
```

### STDIN Support

Read input from standard input for pipeline workflows:

```bash
# Pipe content directly to EpubSmith
cat novel.txt | epub-smith - -o novel.epub

# Use with other CLI tools
grep -v "#" raw.txt | sed 's/\r//g' | epub-smith - -o cleaned.epub
```

### Structure Snapshots

Export chapter structure to a JSON file for reuse and manual adjustment:

```bash
# Export structure snapshot
epub-smith novel.txt --snapshot structure.json

# Use existing structure snapshot
epub-smith novel.txt --use-snapshot structure.json

# Manually edit structure.json to adjust chapters, then re-use
```

### Input Validation (lint mode)

Diagnose text structure issues before conversion:

```bash
# Run lint on a single file
epub-smith lint novel.txt

# Example output:
# 📄 File: novel.txt
# ✅ Chapters detected: 10
# ⚠️  Warning: Found 3 consecutive empty lines at line 150
# ⚠️  Warning: Chapter 5 has duplicate title "Chapter 5"
# ✅ Chapter numbering is consistent
# ⚠️  Warning: Found 2 paragraphs longer than 1000 characters
```

## Command Line Options

EpubSmith provides two command names for convenience:

```
epub-smith [OPTIONS] <INPUT>
```

Or the short name:

```
epbs [OPTIONS] <INPUT>
```

Arguments:
  <INPUT>...  Input TXT file(s) or directory. Use - for STDIN

Options:
  -r, --rules <RULES>        Rules file path
  -o, --output <OUTPUT>      Output EPUB file path [default: book.epub]
  -e, --encoding <ENCODING>  Force input file encoding
      --dry-run              Only show chapter structure, don't generate EPUB
      --print-outline        Output chapter outline
      --explain              Explain parsing process
      --title <TITLE>        Specify book title
      --author <AUTHOR>      Specify book author
      --cover <COVER>        Specify cover image path
      --language <LANGUAGE>  Specify book language [default: zh-CN]
      --check                Validate EPUB with epubcheck
  -v, --verbose              Show detailed logs
      --debug                Enable performance analysis, show timing for each phase
      --lang <LANGUAGE>      Specify output language (e.g., en, zh)
      --export-template <DIRECTORY>  Export style templates to specified directory
      --style <CSS_FILE>     Specify custom CSS style file
      --snapshot <FILE>      Export chapter structure to JSON file
      --use-snapshot <FILE>  Use existing chapter structure from JSON file
  lint                       Run structure validation on input file
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Configuration

### Rules File

You can create a custom rules file in TOML format to control how EpubSmith parses your text files. Here's an example:

```toml
# rules.toml

[meta]
title = "My Book"
author = "John Doe"
language = "en-US"

[rules]
chapter_pattern = "^(Chapter|CHAPTER|第.*章)\s+([0-9]+|[IVXLCDM]+)"

[rules.chapter_number_extraction]
start = 2
end = 3
```

### Using Custom Rules

```bash
epub-smith --rules custom_rules.toml input.txt
```

## Custom Styling

EpubSmith uses Tera templates and CSS for styling. You can export the default templates and modify them:

```bash
epub-smith --export-template my_templates
```

This will create a directory with the default templates and CSS files. You can then modify these files and use them with your custom CSS:

```bash
epub-smith --style my_templates/default.css input.txt
```

## Internationalization

EpubSmith supports multiple languages. You can specify the output language using the `--lang` option:

```bash
epub-smith --lang en input.txt
```

Currently supported languages:
- English (`en`)
- Chinese (`zh`)

## Debugging and Performance

### Dry Run

The `--dry-run` option allows you to preview how EpubSmith will parse your file without generating an EPUB:

```bash
epub-smith --dry-run input.txt
```

### Verbose Logging

Use `--verbose` to see detailed logs during the conversion process:

```bash
epub-smith --verbose input.txt
```

Or with the short name:

```bash
epbs --verbose input.txt
```

### Performance Analysis

The `--debug` option enables performance metrics, showing the time taken for each phase of the conversion:

```bash
epub-smith --debug input.txt
```

Or with the short name:

```bash
epbs --debug input.txt
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
epub-smith novel.txt -o novel.epub

# Step 2: Convert to MOBI using Calibre's ebook-convert
ebook-convert novel.epub novel.mobi

# Step 3: Send to Kindle via email or Calibre
```

### EPUB → PDF

```bash
# Step 1: Generate EPUB with EpubSmith
epub-smith novel.txt -o novel.epub

# Step 2: Convert to PDF using Calibre's ebook-convert
ebook-convert novel.epub novel.pdf

# Or use pandoc for more customization
pandoc novel.epub -o novel.pdf --pdf-engine=xelatex
```

### Batch Processing with Downstream Tools

```bash
# Generate multiple EPUB files first
epub-smith --batch /path/to/txt-files --output-dir /path/to/epub-files

# Then convert all to MOBI in batch
for epub in /path/to/epub-files/*.epub; do
  ebook-convert "$epub" "${epub%.epub}.mobi"
done
```

## Project Structure

```
├── src/
│   ├── parser/          # Text parsing logic
│   ├── renderer/        # EPUB rendering logic
│   ├── resources/       # Default resources (CSS, etc.)
│   ├── templates/       # Tera templates for EPUB generation
│   ├── utils/           # Utility functions
│   │   ├── coherence.rs # Chapter coherence checking
│   │   ├── html.rs      # HTML processing functions
│   │   └── number.rs    # Number conversion functions
│   ├── cli.rs           # Command-line argument parsing
│   ├── config.rs        # Configuration handling
│   ├── export.rs        # Template export functionality
│   ├── lib.rs           # Main library code
│   ├── main.rs          # CLI entry point
│   ├── models.rs        # Data models
│   ├── output.rs        # Output management
│   └── packager.rs      # EPUB packaging logic
├── tests/               # Test suite
├── locales/             # Internationalization files
├── docs/                # Documentation
├── Cargo.toml           # Rust dependencies and configuration
└── README.md            # This file
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Workflow

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes
4. Run the test suite: `cargo test`
5. Run clippy: `cargo clippy`
6. Run rustfmt: `cargo fmt`
7. Submit a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This project uses the [Tera](https://tera.netlify.app/) templating engine
- EPUB generation is powered by the [zip](https://crates.io/crates/zip) crate
- Command-line interface is built with [clap](https://clap.rs/)

## Contact

For questions, suggestions, or issues, please open an [issue](https://github.com/shukang11/epub-smith/issues) on GitHub.
