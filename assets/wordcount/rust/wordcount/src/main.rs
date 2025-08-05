// cargo run -- "C:\Users\phili\OneDrive\Documents\40tude.github.io\docs\06_programmation\rust\011_api_heroku\api_heroku.md"

use regex::Regex;
use std::env;
use std::fs;

fn count_useful_words(md_path: &str, word_limit: usize) -> String {
    let content = match fs::read_to_string(md_path) {
        Ok(c) => c,
        Err(e) => return format!("Error reading file: {}", e),
    };

    // Compile each regex separately for better error handling
    // let patterns = [
    //     (r"(?s)^---.*?---", ""),   // YAML front matter
    //     (r"(?s)<!--.*?-->", ""),   // HTML comments
    //     (r"(?s)```.*?```", ""),    // Code blocks
    //     (r"`[^`]+`", ""),          // Inline code
    //     (r"!?\[.*?\]\(.*?\)", ""), // Images and links
    //     (r"<[^>]+>", ""),          // HTML tags
    //     (r"(?m){:.*?}", ""),       // Special markdown
    //     (r"(?m)\[\^.*?\]:?", ""),  // Footnotes
    //     (r"(?m)^>\s?", ""),        // Blockquotes
    //     (r"(?m)^\|.*?\|$", ""),    // Tables
    //     (r"(?m)^[-*_]{3,}$", ""),  // Horizontal rules
    //     (r"(?s){%.*?%}", ""),      // Template tags
    // ];

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
        (r"(?m)^\|.*?\|$", ""),    // Tables
        (r"(?m)^[-*_]{3,}$", ""),  // Horizontal rules
        (r"(?s)\{\%.*?\%\}", ""),  // Template tags
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <markdown_file.md>", args[0]);
        std::process::exit(1);
    }

    let result = count_useful_words(&args[1], 3000);
    println!("{}", result);
}
