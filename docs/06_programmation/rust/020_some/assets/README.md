# Images Required for some.md Article

This directory should contain 3 images in .webp format:

## 1. option_diagram.webp
**Location in article**: TL;DR section (around line 45)
**Description**: Visual diagram showing:
```
enum Option<T> {
    Some(T),   // Box containing a value
    None,      // Empty box
}
```
**Suggested content**:
- Two boxes side by side
- Left box: "Some(42)" with an actual value inside
- Right box: "None" empty or with ø symbol
- Clean, simple diagram (similar to Rust book style)

**Size**: ~450px width recommended

---

## 2. playground_screenshot.webp
**Location in article**: Introduction section (around line 80)
**Description**: Screenshot of Rust Playground interface showing:
- The code editor with a simple Option example
- "Run" button highlighted or circled
- Output panel showing results
- Browser URL bar showing play.rust-lang.org

**Suggested capture**:
1. Go to https://play.rust-lang.org/
2. Paste a simple example (e.g., Example 1 from article)
3. Click Run
4. Capture the full interface
5. Optionally add annotations (arrows, circles) to highlight key areas

**Size**: ~900px width recommended

---

## 3. cheat_sheet.webp
**Location in article**: Quick Reference section (around line 565)
**Description**: Visual representation of the markdown tables, formatted as an infographic

**Suggested content**:
- Four sections matching the markdown tables:
  1. Extraction Methods (unwrap variants)
  2. Transformation Methods (map, and_then, filter)
  3. Borrowing Methods (as_ref, as_mut, take)
  4. Checking Methods (is_some, is_none)
- Use color coding:
  - Green for safe methods (no panic)
  - Red/orange for panic methods
  - Blue for transformations
- Include "Lazy ✅" or "Eager ⏱️" indicators

**Alternative**: Keep as markdown tables (already readable) - this image is optional

**Size**: If created, ~900px width recommended

---

## How to Create These Images

### Option 1: Manual Design
- Use tools like Figma, Canva, or Excalidraw
- Export as PNG, then convert to .webp:
  ```powershell
  # Using ImageMagick or online converter
  # https://cloudconvert.com/png-to-webp
  ```

### Option 2: Screenshot + Convert
- Take screenshots
- Convert to .webp using online tools or PowerShell:
  ```powershell
  # If you have ImageMagick installed
  magick convert input.png output.webp
  ```

### Option 3: Keep Placeholders
- The article works without images (comments exist)
- Can uncomment image tags later when images are ready

---

## Image Usage in Article

All images use this format:
```markdown
<div align="center">
<img src="./assets/image_name.webp" alt="description" width="NNN" loading="lazy"/>
<br/>
<span>Optional caption</span>
</div>
```

Currently images are commented out in the article. Uncomment after adding .webp files.
