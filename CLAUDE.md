# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

Jekyll static site (40tude.fr) hosted on GitHub Pages using Just the Docs theme. French technical blog covering Rust, Python, C/C++, assembly, OS development, Windows/Linux admin, math, and motorcycle track notes.

## Site Architecture

### Content Structure
- `docs/` - all content organized by category (numbered prefixes for ordering)
  - `02_simple_os/` - OS development series
  - `03_maths/` - math tutorials and proofs
  - `04_windows/`, `05_linux/` - system administration
  - `06_programmation/` - programming content (c/, cpp/, python/, rust/)
  - `07_moto/`, `08_snowboard/` - hobby content
  - `99_divers/` - miscellaneous
- `_layouts/` - custom layouts (mathjax.html for math rendering)
- `_includes/` - custom includes
- `assets/` - site-wide assets
  - `assets/wordcount/` - utility tools (Rust and Python versions)

### Jekyll Configuration
- Theme: just-the-docs 0.10.1 (pinned version)
- Jekyll 4.4.1
- Language: French (fr)
- Google Analytics enabled
- MathJax for math rendering
- Back-to-top button enabled

### Front Matter Pattern
All content pages use:
```yaml
---
published: true
lang: fr  # or en-US for English content
layout: default
title: "Page Title"
description: "Page description for SEO"
parent: "Category Name"
date: YYYY-MM-DD HH:MM:SS
last_modified_date: YYYY-MM-DD HH:MM:SS
---
```

## Development Commands

### Local Development
```powershell
# Serve site locally (Jekyll must be installed)
bundle exec jekyll serve

# Build site
bundle exec jekyll build

# Check word count of markdown file
# Python version
python assets\wordcount\py\wordcount.py "path\to\file.md"

# Rust version (faster)
cd assets\wordcount\rust\wordcount
cargo run -- "C:\path\to\file.md"
```

## Writing Guidelines

### Content Standards
- Target: 3,000 words per article
- Split longer content into series
- Use wordcount tool before publishing
- All content in US English spelling (documentation, code comments, commits)
- UI text and blog content in French

### Markdown Conventions
- Math: use `$$` for inline and display math (single `$` not supported)
- Images: use `{%link %}` for paths on pages with permalinks
- Internal links: use `{%link docs/path/to/file.md%}` or `{%link docs/path/to/file.md%}#anchor`
- Code blocks: specify language for syntax highlighting
- Tables: use Jekyll/kramdown table syntax
- Callouts: `{: .note }`, `{: .warning }`, `{: .important }`, `{: .new }`, `{: .highlight }`

### Math Rendering
- Use `$$\mathrm{\LaTeX}$$` for LaTeX logo
- Use `\mathrm{d}` for upright differential d
- Escape underscores in text mode: `\text{y\_target}`
- Align environments: `\begin{align*}...\end{align*}` (no numbering) or `\begin{align}...\end{align}` (numbered)

### Images and Media
- Convert all images to .webp format
- Use `loading="lazy"` attribute
- Center videos: wrap in `<div align="center">...</div>`
- YouTube embeds: use iframe, wrap in centered div

## Git Workflow

- Main branch: `master`
- Commit messages: concise, sacrifice grammar for brevity
- Use US English in commits
- Site auto-deploys via GitHub Pages on push to master

## Programming Languages in Codebase

### Rust
- Word counting utility in `assets/wordcount/rust/wordcount/`
- Various code examples in `docs/06_programmation/rust/`
- Target: 3000 word limit for articles

### Python
- Word counting utility in `assets/wordcount/py/`
- Code examples in `docs/06_programmation/python/`

### C/C++
- Extensive tutorials and examples in `docs/06_programmation/c/` and `docs/06_programmation/cpp/`
- CVI (LabWindows/CVI) specific content
- "A Tour of C++" chapter-by-chapter notes in `docs/06_programmation/cpp/a_tour_cpp/`

## Common Patterns

### TOC (Table of Contents)
```markdown
# Title
{: .no_toc }

## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}
```

### Callout Blocks
```markdown
{: .note }
> This is a note

{: .warning }
> This is a warning
```

### Custom Anchors
```markdown
## Long Section Title <a id="short-id"></a>

[Link to section](#short-id)
```

## Excluded from Site
- `scrap_book/` - drafts and work in progress
- Various raw_assets/ directories (contain source HTML before markdown conversion)
- Standard Jekyll excludes (Gemfile, etc.)

## SEO and Analytics
- Google Analytics: G-HEDHH8MLQE
- Sitemap generated automatically
- robots.txt in place
- Monitored via Google Search Console
