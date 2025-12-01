# Wordcount

Markdown word counter for blog articles. Filters formatting, code, comments, and syntax to count only prose words.

## Usage

```powershell
cargo run -- C:\Users\phili\OneDrive\Documents\40tude.github.io\docs\06_programmation\rust\020_some\some.md
```

Output:
```
Useful word count: 1952
```

Debug mode to inspect cleaned content:
```powershell
cargo run -- path\to\file.md --debug
```

## About Tests

Run test suite:
```powershell
cargo test
```

The 23 unit and integration tests verify correct filtering of:

**Markdown syntax:**
- YAML front matter (`---...---`)
- Headers (# symbols, preserving text)
- Inline code (backticks)
- Code blocks (triple backticks)
- Links and images (`[text](url)`, `![alt](url)`)
- Blockquotes (`>`)
- Horizontal rules (`---`, `***`, `___`)
- Footnotes (`[^1]`, `[^1]:`)

**Tables:**
- Separator rows (`|---|---|`)
- Alignment markers (`|:---|---:|:---:|`)
- Pipe symbols (replaced with spaces to preserve content)
- Whitespace variations

**Jekyll/kramdown:**
- Special attributes (`{: .note }`, `{: .no_toc }`)
- Template tags (`{% link %}`, `{:toc}`)
- TOC markers (`- TOC`)

**Technical content:**
- HTML comments (`<!-- -->`)
- HTML tags (`<div>`, etc.)
- LaTeX math blocks (`$$...$$`)

**Integration tests:**
- `validation.md`: All patterns combined (101 expected words)
- `validation_01.md`: Tables and headers (14 expected words)
- `table_separators_test.md`: Comprehensive table separator formats (24 expected words)

Tests include edge cases: whitespace variations, alignment markers, multiple columns, single columns, and all six header levels.

## Notes

```powershell
# Find largest markdown file
Get-ChildItem -Path "C:\Users\phili\OneDrive\Documents\40tude.github.io\docs" -Filter *.md -Recurse -File |
    Sort-Object Length -Descending |
    Select-Object -First 1
```

Target: 3000 words per article. Files exceeding this receive split recommendations with excerpts.
