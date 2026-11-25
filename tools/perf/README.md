# Performance bench for www.40tude.fr

Repeatable Lighthouse measurements with a timestamped history, to track the
progress of the performance work. Not published: `tools/` is excluded in
`_config.yml`.

```text
tools/perf/
    Measure-Perf.ps1    entry point (PowerShell 7)
    measure.mjs         measurement logic (Node.js + Lighthouse)
    pages.json          pages to measure
    history.csv         one line per page/profile/measurement: the progress log
    results/            one folder per measurement: HTML reports + summary.md
    docs/               audits and notes
```

## Prerequisites

- PowerShell 7, Node.js LTS (`winget install OpenJS.NodeJS.LTS`), Google Chrome.
- For `-Local` only: Ruby and the site's gems (`bundle install` at the repository root).
- npm dependencies (Lighthouse...) are installed in `tools/perf/node_modules` on first run.

## Usage

From the repository root:

```powershell
# Measure the live site (home + article, mobile + desktop, 3 runs each)
./tools/perf/Measure-Perf.ps1 -Label "baseline"

# Build the working copy with Jekyll and measure it locally, before pushing
./tools/perf/Measure-Perf.ps1 -Local -Label "lazy search"

# Faster check: one page, one profile, one run
./tools/perf/Measure-Perf.ps1 -Local -Pages home -Profiles mobile -Runs 1
```

A full run (2 pages x 2 profiles x 3 runs) takes about 3 minutes.

## Output

- `history.csv`: timestamp (local time with offset), label, target (`live` or
  `local`), git commit (`+dirty` if the site has uncommitted changes), page,
  profile, scores (perf, a11y, best practices, SEO), FCP, LCP, TBT, CLS, Speed
  Index, TTI (ms), total weight and size of `search-data.json` (KiB), report path.
- `results/<date_time>[_local]/`: the HTML report of the median run for each
  page/profile (open it in a browser) and a `summary.md` table.
- The console prints the median results and the delta with the previous
  measurement of the same target/page/profile.

Each measurement adds about 5 MB of HTML reports to `results/`.

## Reading the numbers

- Lighthouse uses simulated throttling (slow 4G + 4x CPU slowdown for mobile),
  so absolute values depend on the machine. Compare measurements made on the
  same PC, and prefer the median of 3 runs over a single run.
- `live` and `local` are not directly comparable: GitHub Pages latency and
  caching differ from the local server. Compare `local` with `local`, `live`
  with `live`.
- TBT (Total Blocking Time) is the metric to watch first: see
  `docs/audit_2026-11-25.md`.

## Adding a page

Add an entry to `pages.json` (`id` is used in file names and in the CSV):

```json
{ "id": "rust", "path": "/docs/06_programmation/rust/" }
```
