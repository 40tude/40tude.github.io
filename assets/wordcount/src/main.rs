// Rust guideline compliant 2025-11-29
// cargo run -- "C:\Users\phili\OneDrive\Documents\40tude.github.io\docs\06_programmation\rust\011_api_heroku\api_heroku.md"

use regex::Regex;
use std::env;
use std::fs;

/// Default word count limit for article splitting recommendations.
///
/// Set to 3000 words based on typical blog article reading time targets.
/// Articles exceeding this limit receive split recommendations with excerpts.
const DEFAULT_WORD_LIMIT: usize = 3000;

/// Counts meaningful words in markdown file after removing metadata and markup.
///
/// Processes markdown content by filtering out YAML front matter, code blocks,
/// HTML comments, inline code, links, images, Jekyll/kramdown tags, LaTeX math,
/// and other non-prose elements. Returns word count and optional split recommendation.
///
/// # Examples
///
/// ```no_run
/// let report = count_useful_words("article.md", 3000);
/// println!("{}", report);
/// // Output: "Useful word count: 2146"
/// ```
///
/// # Errors
///
/// Returns error message string if file cannot be read or regex compilation fails.
///
/// # Panics
///
/// Does not panic under normal operation.
fn count_useful_words(md_path: &str, word_limit: usize) -> String {
    let content = match fs::read_to_string(md_path) {
        Ok(c) => c,
        Err(e) => return format!("Error reading file: {}", e),
    };

    let patterns = [
        (r"(?s)^---.*?---", ""),   // YAML front matter
        (r"(?s)<!--.*?-->", ""),   // HTML comments
        (r"(?s)```.*?```", ""),    // Code blocks
        (r"`[^`]+`", ""),          // Inline code
        (r"!?\[.*?\]\(.*?\)", ""), // Images and links
        (r"<[^>]+>", ""),          // HTML tags
        (r"(?m)\{\:.*?\}", ""),    // Special markdown
        (r"(?m)\[\^.*?\]:?", ""),  // Footnotes
        (r"(?m)^>\s?", ""),        // Blockquotes
        (r"(?m)^\s*\|\s*(?::?-{3,}:?\s*\|)+\s*$", ""), // Tables (separator rows like |---|---| - filtered completely)
        (r"\|", " "),                        // Tables (pipe symbols) - replace with space to preserve words
        (r"(?m)^\s*(#{1,6})\s+", ""),       // Markdown headers (remove # symbols, keep text)
        (r"(?m)^\s*[-*_]{3,}\s*$", ""), // Horizontal rules (with optional whitespace)
        (r"(?s)\{\%.*?\%\}", ""),       // Template tags
        (r"(?s)\$\$.*?\$\$", ""),       // LaTeX math blocks
        (r"(?m)^\s*-\s*TOC\s*$", ""),   // TOC marker (with optional whitespace)
        (r"(?m)^\s*\{:toc\}\s*$", ""),  // Jekyll TOC tag (with optional whitespace)
    ];

    let mut cleaned = content.clone();
    for (pattern, replacement) in patterns.iter() {
        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Warning: Failed to compile regex '{}': {}", pattern, e);
                continue;
            }
        };
        cleaned = re.replace_all(&cleaned, *replacement).into_owned();
    }

    let words: Vec<&str> = cleaned.split_whitespace().collect();
    let word_count = words.len();

    let mut report = format!("Useful word count: {}", word_count);

    if word_count > word_limit {
        let remaining_words = &words[word_limit..];
        let paragraph_start: String = remaining_words
            .iter()
            .take(50)
            .cloned()
            .collect::<Vec<&str>>()
            .join(" ");

        let sentence_end = paragraph_start
            .find(|c| c == '.' || c == '!' || c == '?')
            .map(|pos| &paragraph_start[..=pos])
            .unwrap_or(&paragraph_start);

        report.push_str(&format!(
            "\n\nExcerpt after {} words:\n{}...",
            word_limit, sentence_end
        ));
    }

    report
}

/// Debug version that shows cleaned content.
///
/// Same as count_useful_words but prints cleaned markdown content
/// for manual verification of what is being counted.
fn count_useful_words_debug(md_path: &str, _word_limit: usize) -> String {
    let content = match fs::read_to_string(md_path) {
        Ok(c) => c,
        Err(e) => return format!("Error reading file: {}", e),
    };

    let patterns = [
        (r"(?s)^---.*?---", ""),
        (r"(?s)<!--.*?-->", ""),
        (r"(?s)```.*?```", ""),
        (r"`[^`]+`", ""),
        (r"!?\[.*?\]\(.*?\)", ""),
        (r"<[^>]+>", ""),
        (r"(?m)\{\:.*?\}", ""),
        (r"(?m)\[\^.*?\]:?", ""),
        (r"(?m)^>\s?", ""),
        (r"(?m)^\s*\|\s*(?::?-{3,}:?\s*\|)+\s*$", ""),
        (r"\|", " "),
        (r"(?m)^\s*(#{1,6})\s+", ""),
        (r"(?m)^\s*[-*_]{3,}\s*$", ""),
        (r"(?s)\{\%.*?\%\}", ""),
        (r"(?s)\$\$.*?\$\$", ""),
        (r"(?m)^\s*-\s*TOC\s*$", ""),
        (r"(?m)^\s*\{:toc\}\s*$", ""),
    ];

    let mut cleaned = content.clone();
    for (pattern, replacement) in patterns.iter() {
        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Warning: Failed to compile regex '{}': {}", pattern, e);
                continue;
            }
        };
        cleaned = re.replace_all(&cleaned, *replacement).into_owned();
    }

    eprintln!("=== CLEANED CONTENT ===");
    eprintln!("{}", cleaned);
    eprintln!("=== END CLEANED CONTENT ===\n");

    let words: Vec<&str> = cleaned.split_whitespace().collect();
    let word_count = words.len();

    eprintln!("Words found: {:?}\n", words);

    format!("Useful word count: {}", word_count)
}

/// CLI entry point for markdown word counting utility.
///
/// Accepts markdown file path as first argument, outputs word count report.
/// Use --debug flag to see cleaned content.
///
/// # Examples
///
/// ```sh
/// cargo run -- path/to/article.md
/// # Output: Useful word count: 2146
///
/// cargo run -- path/to/article.md --debug
/// # Shows cleaned content before counting
/// ```
///
/// # Panics
///
/// Exits with code 1 if no file path provided.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <markdown_file.md> [--debug]", args[0]);
        std::process::exit(1);
    }

    let debug = args.contains(&"--debug".to_string());

    if debug {
        let result = count_useful_words_debug(&args[1], DEFAULT_WORD_LIMIT);
        println!("{}", result);
    } else {
        let result = count_useful_words(&args[1], DEFAULT_WORD_LIMIT);
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_yaml_front_matter() {
        let content = r#"---
published: true
lang: en-US
---
This is actual content with five words."#;

        let result = count_useful_words_from_content(content, 1000);
        // Front matter removed, but "This is actual content with five words" = 7 words
        assert_eq!(result, 7, "Should only count words after front matter");
    }

    #[test]
    fn test_html_comments() {
        let content = r#"Before comment
<!-- This is a comment with many words -->
After comment has three words"#;

        let result = count_useful_words_from_content(content, 1000);
        // "Before comment After comment has three words" = 7 words
        assert_eq!(result, 7, "Should exclude HTML comments");
    }

    #[test]
    fn test_code_blocks() {
        let content = r#"Text before
```rust
fn main() {
    println!("lots of code words here");
}
```
Text after has three"#;

        let result = count_useful_words_from_content(content, 1000);
        // "Text before Text after has three" = 6 words
        assert_eq!(result, 6, "Should exclude code blocks");
    }

    #[test]
    fn test_inline_code() {
        let content = "Use the `Option<T>` type for nullable values.";

        let result = count_useful_words_from_content(content, 1000);
        assert_eq!(result, 6, "Should exclude inline code");
    }

    #[test]
    fn test_links_and_images() {
        let content = "Click [here](https://example.com) to continue reading.";

        let result = count_useful_words_from_content(content, 1000);
        assert_eq!(result, 4, "Should exclude links");
    }

    #[test]
    fn test_html_tags() {
        let content = r#"Text <div align="center">inside tags</div> outside."#;

        let result = count_useful_words_from_content(content, 1000);
        assert_eq!(result, 4, "Should exclude HTML tags");
    }

    #[test]
    fn test_jekyll_special_syntax() {
        let content = r#"{: .note }
> This is a note

Regular text here"#;

        let result = count_useful_words_from_content(content, 1000);
        assert_eq!(result, 7, "Should exclude Jekyll syntax markers");
    }

    #[test]
    fn test_footnotes() {
        let content = r#"Some text[^1] with a footnote.

[^1]: This is the footnote content."#;

        let result = count_useful_words_from_content(content, 1000);
        // Footnote removed but [^1] inline reference also removed: "Some text with a footnote. This is the footnote content." = 10 words
        // Actually the regex should remove both inline [^1] and full footnote definition
        assert_eq!(result, 10, "Should exclude footnote definitions");
    }

    #[test]
    fn test_blockquotes() {
        let content = r#"> This is a blockquote with words
> More quoted text

Regular text outside"#;

        let result = count_useful_words_from_content(content, 1000);
        // Blockquote markers removed: "This is a blockquote with words More quoted text Regular text outside" = 12 words
        assert_eq!(result, 12, "Should preserve blockquote content");
    }

    #[test]
    fn test_tables() {
        let content = r#"Before table
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
After table"#;

        let result = count_useful_words_from_content(content, 1000);
        // Separator row filtered, but table content preserved: "Before table Header 1 Header 2 Cell 1 Cell 2 After table" = 12 words
        assert_eq!(result, 12, "Should preserve table content but remove separators and pipes");
    }

    #[test]
    fn test_horizontal_rules() {
        let content = r#"Section one
---
Section two"#;

        let result = count_useful_words_from_content(content, 1000);
        assert_eq!(result, 4, "Should exclude horizontal rules");
    }

    #[test]
    fn test_jekyll_template_tags() {
        let content = r#"Visit {% link docs/page.md %} for more.
Text after has three words."#;

        let result = count_useful_words_from_content(content, 1000);
        // "Visit for more. Text after has three words." = 8 words
        assert_eq!(result, 8, "Should exclude Jekyll template tags");
    }

    #[test]
    fn test_latex_math_blocks() {
        let content = r#"The formula is:
$$
E = mc^2
$$
Simple and elegant."#;

        let result = count_useful_words_from_content(content, 1000);
        // "The formula is: Simple and elegant." = 6 words
        assert_eq!(result, 6, "Should exclude LaTeX math blocks");
    }

    #[test]
    fn test_toc_markers() {
        let content = r#"# Title
{: .no_toc }

## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}

Actual content starts here."#;

        let result = count_useful_words_from_content(content, 1000);
        // After filtering: "Title Table of Contents Actual content starts here." = 8 words (# symbols removed)
        assert_eq!(result, 8, "Should exclude TOC markers and header symbols");
    }

    #[test]
    fn test_combined_patterns() {
        let content = r#"---
title: Test
---
# Article Title
{: .no_toc }

Use `Option<T>` in Rust. See [docs](https://doc.rust-lang.org).

```rust
fn example() {}
```

The math: $$x = y + z$$

Regular content continues here with five words."#;

        let result = count_useful_words_from_content(content, 1000);
        // After all filters: "Article Title Use in Rust. See The math: Regular content continues here with five words."
        // = 16 words (# symbol removed, inline code removed, link removed, code block removed, math removed, front matter removed)
        assert_eq!(result, 16, "Should handle multiple pattern types");
    }

    #[test]
    fn test_integration_validation_file() {
        // Integration test: validates full pipeline with real file I/O
        // File contains all pattern types combined in realistic structure
        let test_file = "test_samples/validation.md";

        // Skip test if file doesn't exist (CI environments, etc.)
        if !std::path::Path::new(test_file).exists() {
            eprintln!("Skipping integration test - {} not found", test_file);
            return;
        }

        let result = super::count_useful_words(test_file, 3000);

        // Extract word count from "Useful word count: N" format
        let count: usize = result
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("Useful word count: "))
            .and_then(|num_str| num_str.parse().ok())
            .expect("Failed to parse word count from result");

        // Expected: Based on validation.md content after all filters applied
        // This is a regression test - if patterns change, update this value
        assert_eq!(count, 101, "Integration test with validation.md");
    }

    #[test]
    fn test_table_separator_standard() {
        // Standard table separator: |--------|--------|
        let content = r#"| Header | Header |
|--------|--------|
| Cell   | Cell   |"#;

        let result = count_useful_words_from_content(content, 1000);
        // "Header Header Cell Cell" = 4 words (separator filtered, pipes removed)
        assert_eq!(result, 4, "Should filter standard table separator");
    }

    #[test]
    fn test_table_separator_long() {
        // Long separator with multiple columns: |--------|--------|--------------|---------------------|
        let content = r#"| Col1 | Col2 | Col3 | Col4 |
|--------|--------|--------------|---------------------|
| Data | Data | Data | Data |"#;

        let result = count_useful_words_from_content(content, 1000);
        // "Col1 Col2 Col3 Col4 Data Data Data Data" = 8 words
        assert_eq!(result, 8, "Should filter long table separator");
    }

    #[test]
    fn test_table_separator_with_whitespace() {
        // Separator with leading/trailing whitespace
        let content = r#"| Header | Header |
   |--------|--------|
| Cell   | Cell   |"#;

        let result = count_useful_words_from_content(content, 1000);
        // "Header Header Cell Cell" = 4 words
        assert_eq!(result, 4, "Should filter table separator with whitespace");
    }

    #[test]
    fn test_table_separator_alignment() {
        // Separator with alignment markers: |:------:|---:|:---|
        let content = r#"| Left | Center | Right |
|:------:|---:|:---|
| L    | C    | R   |"#;

        let result = count_useful_words_from_content(content, 1000);
        // "Left Center Right L C R" = 6 words
        assert_eq!(result, 6, "Should filter table separator with alignment markers");
    }

    #[test]
    fn test_table_separator_single_column() {
        // Single column separator: |--------|
        let content = r#"| Single |
|--------|
| Data   |"#;

        let result = count_useful_words_from_content(content, 1000);
        // "Single Data" = 2 words
        assert_eq!(result, 2, "Should filter single column table separator");
    }

    #[test]
    fn test_integration_validation_01_tables() {
        // Integration test focusing on table handling
        // File contains tables with headers, separators, and content
        let test_file = "test_samples/validation_01.md";

        // Skip test if file doesn't exist
        if !std::path::Path::new(test_file).exists() {
            eprintln!("Skipping integration test - {} not found", test_file);
            return;
        }

        let result = super::count_useful_words(test_file, 3000);

        let count: usize = result
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("Useful word count: "))
            .and_then(|num_str| num_str.parse().ok())
            .expect("Failed to parse word count from result");

        // Expected: 14 words (8 table words + 6 header words)
        // - Table content preserved: One, Two, Three, Four, Five, Six, Seven, Height
        // - Header text preserved (# symbols filtered): Nine, Ten, Eleven, Twelve, Thirteen, Fourteen
        assert_eq!(count, 14, "Integration test with validation_01.md - table and header content preserved, symbols filtered");
    }

    #[test]
    fn test_integration_table_separators() {
        // Integration test focusing on comprehensive table separator patterns
        // File contains 5 different table separator formats with all content in tables
        let test_file = "test_samples/table_separators_test.md";

        // Skip test if file doesn't exist
        if !std::path::Path::new(test_file).exists() {
            eprintln!("Skipping integration test - {} not found", test_file);
            return;
        }

        let result = super::count_useful_words(test_file, 3000);

        let count: usize = result
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("Useful word count: "))
            .and_then(|num_str| num_str.parse().ok())
            .expect("Failed to parse word count from result");

        // Expected: 24 words from 5 tables (all separators filtered, content preserved)
        // Test 1 (standard): Header, Header, Cell, Cell = 4 words
        // Test 2 (long): Col1, Col2, Col3, Col4, Data, Data, Data, Data = 8 words
        // Test 3 (whitespace): Header, Header, Cell, Cell = 4 words
        // Test 4 (alignment): Left, Center, Right, L, C, R = 6 words
        // Test 5 (single column): Single, Data = 2 words
        assert_eq!(count, 24, "Integration test with table_separators_test.md - all separator types handled correctly");
    }

    /// Helper function for test module that counts words from content string.
    ///
    /// Duplicates main logic for testing without file I/O. Applies same regex
    /// patterns as production code.
    fn count_useful_words_from_content(content: &str, _word_limit: usize) -> usize {
        use regex::Regex;

        let patterns = [
            (r"(?s)^---.*?---", ""),
            (r"(?s)<!--.*?-->", ""),
            (r"(?s)```.*?```", ""),
            (r"`[^`]+`", ""),
            (r"!?\[.*?\]\(.*?\)", ""),
            (r"<[^>]+>", ""),
            (r"(?m)\{\:.*?\}", ""),
            (r"(?m)\[\^.*?\]:?", ""),
            (r"(?m)^>\s?", ""),
            (r"(?m)^\s*\|\s*(?::?-{3,}:?\s*\|)+\s*$", ""),
            (r"\|", " "),
            (r"(?m)^\s*(#{1,6})\s+", ""),
            (r"(?m)^\s*[-*_]{3,}\s*$", ""),
            (r"(?s)\{\%.*?\%\}", ""),
            (r"(?s)\$\$.*?\$\$", ""),
            (r"(?m)^\s*-\s*TOC\s*$", ""),
            (r"(?m)^\s*\{:toc\}\s*$", ""),
        ];

        let mut cleaned = content.to_string();
        for (pattern, replacement) in patterns.iter() {
            let re = Regex::new(pattern).unwrap();
            cleaned = re.replace_all(&cleaned, *replacement).into_owned();
        }

        cleaned.split_whitespace().count()
    }
}
