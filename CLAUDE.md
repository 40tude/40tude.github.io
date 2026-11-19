# 40tude.fr blog

Personal blog, Jekyll + Just the Docs theme, hosted on GitHub Pages (www.40tude.fr).
Push to `master` = publish. Never commit or push unless asked.
Author's full editing cheat sheet (French): `README.md`. Read the relevant section before unusual edits.

## Language
- Article text follows the article: its `lang` front matter and existing prose (most `en-US`, some `fr`). This overrides the global "US English artifacts" rule for article content.
- Keep the author's voice and tone. Review = targeted fixes, not rewrites. Propose before rewriting a whole section.
- Spelling, grammar, and typography (incl. FR spacing rules) are checked by LTeX+ in VS Code. Custom words: `.vscode/ltex.dictionary.fr.txt`.

## Writing style
- Conversational, pedagogical, direct. Explain the "why", not just the "how".
- FR articles: "je" for the author, "vous" for the reader.
- EN articles: inclusive "we" ("we could put everything in a single file").
- Short paragraphs. Bold key terms, italics for nuance.
- Typical skeleton: TL;DR bullets, hero image, TOC, prerequisites, body, conclusion.
- Concrete analogies and examples over abstract statements.

## Code blocks
- Code, comments, identifiers, and CLI output: always US English, whatever the article language.
- Commands: PowerShell unless the article targets Linux.
- Always a language tag (`powershell`, `rust`, `python`, `text`...).

## Layout
```text
docs/<NN_category>/                 category, has index.md (its title = articles' `parent`)
    <NNN_slug>/
        <slug>.md                   article
        assets/                     published files, images as .webp (img00.webp, img01.webp...)
        raw_assets/                 git-ignored, OneDrive only (sources, notebooks, .venv)
assets/_article_template.md         article template
tools/                              scripts, not published (excluded in _config.yml)
scrap_book/                         drafts, not published
```
- New article: `tools/New-Post.ps1` run from a category folder (`new-post my_slug`). Do not hand-build the folder.
- Target ~3,000 words per article; suggest a series beyond that.

## Front matter
- Never edit `last_modified_date`: the pre-commit hook `tools/hooks/pre-commit` sets it (`core.hooksPath = tools/hooks`).
- Never change `date` of an existing article.
- `image`: full path from repo root (`docs/.../assets/img00.webp`). Also used for Open Graph.
- `published: false` hides a draft.

## Markdown / kramdown rules
- Internal links: `[text]({%link docs/<path>/<file>.md%})`, optional `#anchor`. Never raw relative `.md` links between articles.
- External links opening in a new tab: `[text](url){:target="_blank"}`.
- Math (MathJax 3): always `$$...$$`, inline in a sentence or alone on its line for display. Never single `$`.
- Inside `\text{}` no commands: write `\text{ en } \%`, not `\text{ en \%}`.
- Underscore in math: `$$y\_target$$`.
- Title: `# Title` + `{: .no_toc }`, then lead paragraph + `{: .lead }`.
- TOC covers level-2 headings only: add `{: .no_toc }` under every `###` and deeper.
- Callouts: `{: .note }`, `{: .warning }`, `{: .important }`, `{: .new }`, `{: .highlight }` before a `>` block.
- Images: `<figure>` + `<img src="./assets/imgNN.webp" alt="..." loading="lazy"/>` + `<figcaption>`. Copy the pattern already used in the article.
- Code that contains `{{` or `{%`: wrap in `{% raw %}` ... `{% endraw %}` or Jekyll will parse it.
- Footnotes: `[^1]` in text, definitions at the end after `----`.

## Do not touch without asking
- `_config.yml`, `_layouts/`, `_includes/`, `Gemfile*`
- `.crossnote/parser.js` (VS Code MPE preview only, not the site)
- `tools/hooks/` (must stay LF, see `.gitattributes`)
