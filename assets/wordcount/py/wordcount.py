# python wordcount.py <markdown_file.md>
import re
from pathlib import Path
from typing import Optional


def count_useful_words(md_path: str, word_limit: int = 3000) -> str:
    try:
        content = Path(md_path).read_text(encoding="utf-8")
    except Exception as e:
        return f"Error reading file: {str(e)}"

    # Remove YAML front matter (between ---)
    content = re.sub(r"^---[\s\S]*?---", "", content, count=1)

    # Remove HTML comments (multi-line and single-line)
    content = re.sub(r"<!--[\s\S]*?-->", "", content)

    # Remove all code blocks (```lang and ```)
    content = re.sub(r"```[\s\S]*?```", "", content)

    # Remove inline code snippets
    content = re.sub(r"`[^`]+`", "", content)

    # Remove images and links
    content = re.sub(r"!?\[.*?\]\(.*?\)", "", content)

    # Remove HTML tags
    content = re.sub(r"<[^>]+>", "", content)

    # Remove special Markdown formatting like {: .class }
    content = re.sub(r"{:.*?}", "", content)

    # Remove footnotes and reference-style links
    content = re.sub(r"\[\^.*?\]:?", "", content)

    # Remove blockquotes markers
    content = re.sub(r"^>\s?", "", content, flags=re.MULTILINE)

    # Remove tables (simple detection)
    content = re.sub(r"^\|.*?\|$", "", content, flags=re.MULTILINE)

    # Remove horizontal rules
    content = re.sub(r"^[-*_]{3,}$", "", content, flags=re.MULTILINE)

    # Remove metadata tags (like Jekyll tags)
    content = re.sub(r"{%[\s\S]*?%}", "", content)

    # Remove multiple spaces and newlines
    content = re.sub(r"\s+", " ", content).strip()

    # Remove punctuation (optional, can be commented out if needed)
    cleaned = re.sub(r"[^\w\s'-]", "", content)

    words = cleaned.split()
    word_count = len(words)

    report = f"Useful word count: {word_count}"

    if word_count > word_limit:
        remaining_words = words[word_limit:]
        paragraph_start = " ".join(remaining_words[:50])  # Take first 50 words
        # Find the first sentence end if possible
        match = re.search(r"^.*?[.!?]\s", paragraph_start)
        if match:
            paragraph_start = match.group(0).strip() + "..."
        else:
            paragraph_start = paragraph_start + "..."

        report += f"\n\nExcerpt after {word_limit} words:\n{paragraph_start}"

    return report


if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Usage: python wordcount.py <markdown_file.md>")
        sys.exit(1)

    result = count_useful_words(sys.argv[1])
    print(result)
