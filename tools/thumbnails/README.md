# Home page thumbnails

The home page (`index.md`) lists the 10 most recently modified articles, each
with a 150 px picture. Instead of the full-size hero image (often 100-180 KB),
it shows a 300 px wide WebP thumbnail (about 10 KB) stored next to the image:

```text
docs/<category>/<article>/assets/img00.webp          hero image
docs/<category>/<article>/assets/img00_thumb.webp    its thumbnail (generated)
```

The hero image is the one `index.md` already uses:

1. the front matter `image:` (path from the repository root),
2. otherwise the first `<img src="...">` of the article.

If no thumbnail exists, `index.md` falls back to the original image.

## Automatic generation (pre-commit hook)

`tools/hooks/pre-commit` runs `make-thumbnails.mjs` on the staged files:

- any staged article (`docs/**/*.md`, new or updated): its thumbnail is created
  if missing, or refreshed if the hero image is newer;
- any staged image that already has a thumbnail: the thumbnail is refreshed.

Generated thumbnails are added to the same commit. If Node.js or the npm
packages are missing, the hook prints a warning and the commit goes on.

## Setup (once per machine)

```powershell
# From the repository root. Node.js LTS is required (winget install OpenJS.NodeJS.LTS)
npm ci --prefix tools/thumbnails
git config core.hooksPath tools/hooks   # already set if last_modified_date is updated on commit
```

## Manual runs

```powershell
# All articles (only missing or outdated thumbnails are written)
node tools/thumbnails/make-thumbnails.mjs --all

# One article
node tools/thumbnails/make-thumbnails.mjs docs/09_ml_ai/009_how_attention_works/how_attention_works_00.md
```

Settings (width 300 px, WebP quality 75) are constants at the top of
`make-thumbnails.mjs`. After changing them, delete the `*_thumb.webp` files and
run `--all` again.
