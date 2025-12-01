---
title: Validation Test
published: true
---

# Article Title
{: .no_toc }

## Table of Contents
{: .no_toc }
- TOC
{:toc}

This is a simple paragraph with exactly ten words here now.

Here is some `inline code` that should be excluded.

```rust
fn main() {
    println!("This code block should not count");
}
```

Visit [this link](https://example.com) for more information.

<!-- This HTML comment has many words that should disappear -->

The formula is $$E = mc^2$$ which should be filtered.

<div>HTML tags should also disappear completely</div>

> Blockquote text that should remain and be counted properly.

| Header | Should | Header_Long       | Should_Long      |
|--------|--------|--------------|---------------------|
| Be     | Gone   | Be           | Gone     |

Final sentence with five clear words.

[^1]: Footnote that should not count at all.

Some text with footnote reference[^1] here.


#     Level 1
##    Level 2
###   Level 3
####  Level 4
##### Level 5

<!--

Expected word count after all filters: 86 words

-->
