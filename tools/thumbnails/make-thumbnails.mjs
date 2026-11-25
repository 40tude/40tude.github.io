// Generates the home page thumbnail of each article: <hero>_thumb.webp,
// 300 px wide, written next to the hero image in the article's assets/ folder.
//
// The hero image is found with the same rule as index.md:
//   1. front matter `image:` (path from the repository root)
//   2. otherwise the src of the first <img in the article body
// A thumbnail is (re)generated when it is missing or older than its source.
//
// Usage (from anywhere):
//   node tools/thumbnails/make-thumbnails.mjs --all            all docs/**/*.md
//   node tools/thumbnails/make-thumbnails.mjs <file> [<file>]  some articles (.md)
//                                                              and/or images
// Image arguments only refresh an existing thumbnail of that image (used by
// the pre-commit hook when a hero image is edited).
// Prints one line per thumbnail written: "THUMB <path from repo root>".

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import sharp from 'sharp';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = path.resolve(HERE, '..', '..');
const THUMB_WIDTH = 300; // displayed at 150 px on the home page, x2 for high-DPI screens
const THUMB_QUALITY = 75;
const THUMB_SUFFIX = '_thumb.webp';
const RASTER = /\.(webp|png|jpe?g|gif|avif|tiff?)$/i;

// Path from the repository root, always with forward slashes
const rel = (p) => path.relative(REPO_ROOT, p).split(path.sep).join('/');

export function thumbPathFor(imagePath) {
  return imagePath.replace(/\.[^./\\]+$/, '') + THUMB_SUFFIX;
}

// Returns the absolute path of the article's hero image, or null
export function findHeroImage(mdFile) {
  const text = fs.readFileSync(mdFile, 'utf8');
  let body = text;
  const fm = text.match(/^---\s*\r?\n([\s\S]*?)\r?\n---\s*(\r?\n|$)/);
  if (fm) {
    body = text.slice(fm[0].length);
    const img = fm[1].match(/^image:\s*["']?([^"'\r\n#]+?)["']?\s*$/m);
    if (img && img[1].trim() !== '') {
      return path.resolve(REPO_ROOT, img[1].trim().replace(/^\//, ''));
    }
  }
  // Same parsing as index.md: split on '<img', then take src="..." in the first tag
  const parts = body.split('<img');
  if (parts.length < 2) return null;
  const src = parts[1].split('src="');
  if (src.length < 2) return null;
  let url = src[1].split('"')[0].trim();
  if (url === '' || /^(https?:)?\/\//i.test(url) || url.startsWith('data:') || url.includes('{{')) return null;
  url = url.split(/[?#]/)[0];
  try { url = decodeURI(url); } catch { /* keep as is */ }
  return url.startsWith('/')
    ? path.resolve(REPO_ROOT, url.slice(1))
    : path.resolve(path.dirname(mdFile), url);
}

async function makeThumb(image, { onlyIfExists = false } = {}) {
  if (!RASTER.test(image) || image.endsWith(THUMB_SUFFIX)) return null;
  if (!fs.existsSync(image)) {
    if (!onlyIfExists) console.warn(`WARN  hero image not found: ${rel(image)}`);
    return null;
  }
  const thumb = thumbPathFor(image);
  const exists = fs.existsSync(thumb);
  if (onlyIfExists && !exists) return null;
  if (exists && fs.statSync(thumb).mtimeMs >= fs.statSync(image).mtimeMs) return null;
  await sharp(image, { animated: false })
    .resize({ width: THUMB_WIDTH, withoutEnlargement: true })
    .webp({ quality: THUMB_QUALITY, effort: 6 })
    .toFile(thumb);
  console.log(`THUMB ${rel(thumb)}`);
  return thumb;
}

function listArticles(dir) {
  const out = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === 'raw_assets' || entry.name === 'node_modules' || entry.name.startsWith('.')) continue;
      out.push(...listArticles(p));
    } else if (entry.name.endsWith('.md')) {
      out.push(p);
    }
  }
  return out;
}

async function main() {
  const args = process.argv.slice(2);
  if (args.length === 0) {
    console.error('Usage: make-thumbnails.mjs --all | <article.md|image> ...');
    process.exit(2);
  }
  const files = args.includes('--all')
    ? listArticles(path.join(REPO_ROOT, 'docs'))
    : args.map((a) => path.resolve(process.cwd(), a));

  let written = 0;
  let failed = 0;
  for (const file of files) {
    try {
      if (file.endsWith('.md')) {
        if (!fs.existsSync(file)) continue;
        const hero = findHeroImage(file);
        if (hero && await makeThumb(hero)) written++;
      } else if (await makeThumb(file, { onlyIfExists: true })) {
        written++;
      }
    } catch (err) {
      failed++;
      console.warn(`WARN  ${rel(file)}: ${err.message}`);
    }
  }
  console.error(`${written} thumbnail(s) written${failed ? `, ${failed} error(s)` : ''}.`);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main();
}
