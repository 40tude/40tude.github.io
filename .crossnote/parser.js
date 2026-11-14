// Markdown Preview Enhanced (crossnote) hooks for this workspace.
// Not used by Jekyll (dot folders are ignored by the site build).
//
// Why: kramdown (Jekyll) treats $$...$$ as INLINE math when it sits inside a
// line of text, and as DISPLAY math only when it stands alone. MPE always
// renders $$...$$ as display math, which breaks paragraphs in the preview.
// Fix: before MPE parses the file, rewrite inline $$...$$ to $...$.
// Untouched: code fences, inline code, lines that are only $$...$$, and
// multi-line $$ blocks.
({
  onWillParseMarkdown: async function (markdown) {
    const lines = markdown.split("\n");
    let inFence = false;
    let inMathBlock = false;
    const fenceRe = /^\s*(```|~~~)/;
    const displayLineRe = /^\s*\$\$(?:(?!\$\$).)*\$\$\s*$/;
    const inlineRe = /\$\$((?:(?!\$\$).)+?)\$\$/g;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      if (fenceRe.test(line)) {
        inFence = !inFence;
        continue;
      }
      if (inFence) continue;
      if (line.trim() === "$$") {
        inMathBlock = !inMathBlock;
        continue;
      }
      if (inMathBlock) continue;
      if (displayLineRe.test(line)) continue;
      if (line.indexOf("$$") === -1) continue;

      // Rewrite only outside inline code spans
      lines[i] = line
        .split(/(`[^`]*`)/)
        .map((part, idx) =>
          idx % 2 === 1
            ? part
            : part.replace(inlineRe, (_, tex) => "$" + tex.trim() + "$")
        )
        .join("");
    }
    return lines.join("\n");
  },

  onDidParseMarkdown: async function (html) {
    return html;
  },
});
