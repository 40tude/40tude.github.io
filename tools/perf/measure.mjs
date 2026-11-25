// Lighthouse measurement bench for www.40tude.fr.
//
// Measures every page listed in pages.json, in mobile and desktop profiles,
// N runs each, keeps the median run (Lighthouse's own median algorithm),
// saves its HTML report under results/<timestamp>/ and appends one line per
// page/profile to history.csv so progress can be tracked over time.
//
// Usually started through Measure-Perf.ps1, but can be run directly:
//   node measure.mjs [--target live|local] [--label "text"] [--runs 3]
//                    [--profiles mobile,desktop] [--pages home,article]
//   node measure.mjs --dashboard-only    (only regenerate dashboard.html)

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import http from 'node:http';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import compression from 'compression';
import * as chromeLauncher from 'chrome-launcher';
import lighthouse from 'lighthouse';
import desktopConfig from 'lighthouse/core/config/desktop-config.js';
import { computeMedianRun } from 'lighthouse/core/lib/median-run.js';
import serveHandler from 'serve-handler';

import { writeDashboard } from './dashboard.mjs';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = path.resolve(HERE, '..', '..');
const RESULTS_DIR = path.join(HERE, 'results');
const HISTORY_CSV = path.join(HERE, 'history.csv');
const DASHBOARD_HTML = path.join(HERE, 'dashboard.html');
const LOCAL_PORT = 4848;

const CSV_COLUMNS = [
  'timestamp', 'label', 'target', 'commit', 'machine', 'lighthouse', 'page', 'profile', 'runs',
  'perf', 'a11y', 'best_practices', 'seo',
  'fcp_ms', 'lcp_ms', 'tbt_ms', 'cls', 'si_ms', 'tti_ms',
  'total_kib', 'requests', 'search_data_kib', 'report',
];

// ---------------------------------------------------------------------------
// Command line

function parseArgs(argv) {
  const opts = { target: 'live', label: '', runs: 3, profiles: ['mobile', 'desktop'], pages: null, dashboardOnly: false };
  for (let i = 0; i < argv.length; i++) {
    const key = argv[i];
    const next = () => {
      if (i + 1 >= argv.length) throw new Error(`Missing value for ${key}`);
      return argv[++i];
    };
    switch (key) {
      case '--target': opts.target = next(); break;
      case '--label': opts.label = next(); break;
      case '--runs': opts.runs = Number.parseInt(next(), 10); break;
      case '--profiles': opts.profiles = next().split(',').map((s) => s.trim()).filter(Boolean); break;
      case '--pages': opts.pages = next().split(',').map((s) => s.trim()).filter(Boolean); break;
      case '--dashboard-only': opts.dashboardOnly = true; break;
      default: throw new Error(`Unknown argument: ${key}`);
    }
  }
  if (!['live', 'local'].includes(opts.target)) throw new Error('--target must be "live" or "local"');
  if (!Number.isInteger(opts.runs) || opts.runs < 1) throw new Error('--runs must be a positive integer');
  for (const p of opts.profiles) {
    if (!['mobile', 'desktop'].includes(p)) throw new Error(`Unknown profile: ${p}`);
  }
  return opts;
}

// ---------------------------------------------------------------------------
// Helpers

function timestampParts(d = new Date()) {
  const pad = (n) => String(n).padStart(2, '0');
  const date = `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
  const time = `${pad(d.getHours())}${pad(d.getMinutes())}${pad(d.getSeconds())}`;
  // Local time with UTC offset, e.g. 2026-11-25T20:49:03+01:00
  const offMin = -d.getTimezoneOffset();
  const sign = offMin >= 0 ? '+' : '-';
  const iso = `${date}T${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`
    + `${sign}${pad(Math.floor(Math.abs(offMin) / 60))}:${pad(Math.abs(offMin) % 60)}`;
  return { folder: `${date}_${time}`, iso };
}

function gitCommit() {
  const rev = spawnSync('git', ['rev-parse', '--short', 'HEAD'], { cwd: REPO_ROOT, encoding: 'utf8' });
  if (rev.status !== 0) return 'unknown';
  // Only changes outside tools/perf make the measured site differ from the commit
  const st = spawnSync('git', ['status', '--porcelain', '--', '.', ':(exclude)tools/perf'], { cwd: REPO_ROOT, encoding: 'utf8' });
  const dirty = st.status === 0 && st.stdout.trim() !== '';
  return rev.stdout.trim() + (dirty ? '+dirty' : '');
}

function csvField(value) {
  const s = value === undefined || value === null ? '' : String(value);
  return /[",\r\n]/.test(s) ? `"${s.replace(/"/g, '""')}"` : s;
}

function parseCsvLine(line) {
  const out = [];
  let cur = '';
  let quoted = false;
  for (let i = 0; i < line.length; i++) {
    const c = line[i];
    if (quoted) {
      if (c === '"' && line[i + 1] === '"') { cur += '"'; i++; }
      else if (c === '"') quoted = false;
      else cur += c;
    } else if (c === '"') quoted = true;
    else if (c === ',') { out.push(cur); cur = ''; }
    else cur += c;
  }
  out.push(cur);
  return out;
}

function readHistory() {
  if (!fs.existsSync(HISTORY_CSV)) return [];
  const lines = fs.readFileSync(HISTORY_CSV, 'utf8').split(/\r?\n/).filter((l) => l.trim() !== '');
  if (lines.length < 2) return [];
  const header = parseCsvLine(lines[0]);
  return lines.slice(1).map((l) => {
    const cells = parseCsvLine(l);
    return Object.fromEntries(header.map((h, i) => [h, cells[i]]));
  });
}

function toCsvLines(rows) {
  return rows.map((r) => CSV_COLUMNS.map((c) => csvField(r[c])).join(','));
}

function appendHistory(rows) {
  const exists = fs.existsSync(HISTORY_CSV) && fs.statSync(HISTORY_CSV).size > 0;
  if (exists) {
    const header = fs.readFileSync(HISTORY_CSV, 'utf8').split(/\r?\n/, 1)[0];
    if (header !== CSV_COLUMNS.join(',')) {
      // Older file layout: rewrite it with the current columns (missing values stay empty)
      const old = readHistory();
      fs.writeFileSync(HISTORY_CSV, [CSV_COLUMNS.join(','), ...toCsvLines(old), ...toCsvLines(rows)].join('\n') + '\n', 'utf8');
      return;
    }
  }
  const text = (exists ? '' : CSV_COLUMNS.join(',') + '\n') + toCsvLines(rows).join('\n') + '\n';
  fs.appendFileSync(HISTORY_CSV, text, 'utf8');
}

// ---------------------------------------------------------------------------
// Local mode: build the site with Jekyll, then serve it with gzip compression
// (GitHub Pages compresses too, so transfer sizes stay comparable).

function buildLocalSite() {
  const dest = path.join(os.tmpdir(), '40tude-perf-site');
  console.log(`Building the site with Jekyll into ${dest} ...`);
  const env = { ...process.env, JEKYLL_ENV: 'production' };
  const res = spawnSync('bundle', ['exec', 'jekyll', 'build', '-d', dest], {
    cwd: REPO_ROOT, env, stdio: ['ignore', 'ignore', 'pipe'], encoding: 'utf8', shell: process.platform === 'win32',
  });
  if (res.status !== 0) {
    // Jekyll prints tons of Sass deprecation warnings on stderr, keep only the tail
    console.error(res.stderr.split(/\r?\n/).slice(-15).join('\n'));
    throw new Error('Jekyll build failed (is Ruby + "bundle install" set up?)');
  }
  return dest;
}

function startLocalServer(root) {
  const compress = compression();
  const server = http.createServer((req, res) => {
    compress(req, res, () => serveHandler(req, res, { public: root, cleanUrls: true }));
  });
  return new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(LOCAL_PORT, '127.0.0.1', () => resolve(server));
  });
}

// ---------------------------------------------------------------------------
// Lighthouse

function extractMetrics(lhr) {
  const a = lhr.audits;
  const num = (id) => (a[id] && typeof a[id].numericValue === 'number' ? a[id].numericValue : null);
  const score = (id) => (lhr.categories[id] ? Math.round(lhr.categories[id].score * 100) : null);
  const requests = a['network-requests']?.details?.items ?? [];
  const searchData = requests.find((r) => r.url.includes('search-data.json'));
  return {
    perf: score('performance'),
    a11y: score('accessibility'),
    best_practices: score('best-practices'),
    seo: score('seo'),
    fcp_ms: Math.round(num('first-contentful-paint')),
    lcp_ms: Math.round(num('largest-contentful-paint')),
    tbt_ms: Math.round(num('total-blocking-time')),
    cls: Number((num('cumulative-layout-shift') ?? 0).toFixed(3)),
    si_ms: Math.round(num('speed-index')),
    tti_ms: Math.round(num('interactive')),
    total_kib: Math.round((num('total-byte-weight') ?? 0) / 1024),
    requests: requests.length,
    search_data_kib: searchData ? Math.round(searchData.transferSize / 1024) : 0,
  };
}

async function runLighthouse(url, profile) {
  const chrome = await chromeLauncher.launch({
    chromeFlags: ['--headless=new', '--no-sandbox', '--disable-gpu'],
  });
  try {
    const flags = {
      port: chrome.port,
      output: 'html',
      logLevel: 'error',
      onlyCategories: ['performance', 'accessibility', 'best-practices', 'seo'],
    };
    const config = profile === 'desktop' ? desktopConfig : undefined;
    const result = await lighthouse(url, flags, config);
    if (!result || !result.lhr) throw new Error(`Lighthouse returned no result for ${url}`);
    if (result.lhr.runtimeError) throw new Error(`${url}: ${result.lhr.runtimeError.message}`);
    return result;
  } finally {
    // On Windows, chrome-launcher sometimes fails to delete its temp profile (EPERM): harmless
    try { await chrome.kill(); } catch { /* ignore */ }
  }
}

// ---------------------------------------------------------------------------
// Console output

function fmtMs(v) { return v === '' || v === undefined || v === null ? '-' : `${(Number(v) / 1000).toFixed(2)}s`; }

function delta(cur, prev, lowerIsBetter) {
  if (prev === undefined || prev === null || prev === '') return '';
  const d = Number(cur) - Number(prev);
  if (d === 0) return ' (=)';
  const better = lowerIsBetter ? d < 0 : d > 0;
  const sign = d > 0 ? '+' : '';
  return ` (${sign}${Number.isInteger(d) ? d : d.toFixed(3)}${better ? ' better' : ' worse'})`;
}

function printSummary(rows, previous) {
  console.log('\nResults (median run):');
  for (const r of rows) {
    const p = previous.get(`${r.target}|${r.page}|${r.profile}`);
    console.log(`\n  ${r.page} / ${r.profile}`);
    console.log(`    Perf ${r.perf}${delta(r.perf, p?.perf, false)}`
      + `  A11y ${r.a11y}  BP ${r.best_practices}  SEO ${r.seo}`);
    console.log(`    FCP ${fmtMs(r.fcp_ms)}  LCP ${fmtMs(r.lcp_ms)}${delta(r.lcp_ms, p?.lcp_ms, true)}`
      + `  TBT ${r.tbt_ms}ms${delta(r.tbt_ms, p?.tbt_ms, true)}  CLS ${r.cls}  SI ${fmtMs(r.si_ms)}`);
    console.log(`    Weight ${r.total_kib} KiB${delta(r.total_kib, p?.total_kib, true)}`
      + `  Requests ${r.requests}  search-data.json ${r.search_data_kib} KiB`);
  }
  if (previous.size > 0) console.log('\n  Deltas compare with the previous run of the same target/page/profile on this machine.');
}

function writeRunSummary(dir, meta, rows) {
  const lines = [
    `# Performance run ${meta.iso}`,
    '',
    `- Label: ${meta.label || '(none)'}`,
    `- Target: ${meta.target} (${meta.baseUrl})`,
    `- Commit: ${meta.commit}`,
    `- Runs per page/profile: ${meta.runs} (median kept)`,
    `- Lighthouse: ${meta.lighthouseVersion}, ${meta.userAgent}`,
    '',
    '| Page | Profile | Perf | FCP | LCP | TBT | CLS | SI | Weight | Requests | search-data.json | Report |',
    '|---|---|---|---|---|---|---|---|---|---|---|---|',
    ...rows.map((r) => `| ${r.page} | ${r.profile} | ${r.perf} | ${fmtMs(r.fcp_ms)} | ${fmtMs(r.lcp_ms)} | ${r.tbt_ms} ms | ${r.cls} | ${fmtMs(r.si_ms)} | ${r.total_kib} KiB | ${r.requests} | ${r.search_data_kib} KiB | [html](${path.basename(r.report)}) |`),
    '',
  ];
  fs.writeFileSync(path.join(dir, 'summary.md'), lines.join('\n'), 'utf8');
}

// ---------------------------------------------------------------------------
// Main

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  if (opts.dashboardOnly) {
    writeDashboard(readHistory(), DASHBOARD_HTML);
    console.log(`Dashboard: ${path.relative(process.cwd(), DASHBOARD_HTML) || DASHBOARD_HTML}`);
    return;
  }
  const cfg = JSON.parse(fs.readFileSync(path.join(HERE, 'pages.json'), 'utf8'));
  let pages = cfg.pages;
  if (opts.pages) {
    pages = pages.filter((p) => opts.pages.includes(p.id));
    if (pages.length === 0) throw new Error(`No page matches --pages ${opts.pages.join(',')}`);
  }

  const { folder, iso } = timestampParts();
  const commit = gitCommit();
  let baseUrl = cfg.liveBaseUrl.replace(/\/$/, '');
  let server = null;

  if (opts.target === 'local') {
    const root = buildLocalSite();
    server = await startLocalServer(root);
    baseUrl = `http://localhost:${LOCAL_PORT}`;
  }

  const runDir = path.join(RESULTS_DIR, folder + (opts.target === 'local' ? '_local' : ''));
  fs.mkdirSync(runDir, { recursive: true });

  const previous = new Map();
  // Compare with the last run of the same target/page/profile on this machine
  // (rows written before the "machine" column existed match any machine)
  const host = os.hostname();
  for (const h of readHistory()) {
    if (h.machine && h.machine !== host) continue;
    previous.set(`${h.target}|${h.page}|${h.profile}`, h);
  }

  const rows = [];
  let lighthouseVersion = '';
  let userAgent = '';
  try {
    for (const page of pages) {
      for (const profile of opts.profiles) {
        const url = baseUrl + page.path;
        const results = [];
        for (let i = 1; i <= opts.runs; i++) {
          process.stdout.write(`  ${page.id} / ${profile}: run ${i}/${opts.runs} ...`);
          const res = await runLighthouse(url, profile);
          results.push(res);
          console.log(` perf ${Math.round(res.lhr.categories.performance.score * 100)}, `
            + `TBT ${Math.round(res.lhr.audits['total-blocking-time'].numericValue)} ms`);
        }
        const median = computeMedianRun(results.map((r) => r.lhr));
        const best = results.find((r) => r.lhr === median) ?? results[0];
        lighthouseVersion = best.lhr.lighthouseVersion;
        userAgent = best.lhr.environment?.hostUserAgent ?? '';

        const reportFile = path.join(runDir, `${page.id}_${profile}.html`);
        fs.writeFileSync(reportFile, best.report, 'utf8');

        rows.push({
          timestamp: iso,
          label: opts.label,
          target: opts.target,
          commit,
          machine: os.hostname(),
          lighthouse: best.lhr.lighthouseVersion,
          page: page.id,
          profile,
          runs: opts.runs,
          ...extractMetrics(best.lhr),
          report: path.relative(HERE, reportFile).split(path.sep).join('/'),
        });
      }
    }
  } finally {
    if (server) server.close();
  }

  appendHistory(rows);
  writeRunSummary(runDir, {
    iso, label: opts.label, target: opts.target, baseUrl, commit, runs: opts.runs, lighthouseVersion,
    userAgent: userAgent.replace(/\(.*?\)/, '').trim(),
  }, rows);
  writeDashboard(readHistory(), DASHBOARD_HTML);
  printSummary(rows, previous);
  console.log(`\nReports: ${path.relative(process.cwd(), runDir) || runDir}`);
  console.log(`History: ${path.relative(process.cwd(), HISTORY_CSV) || HISTORY_CSV}`);
  console.log(`Dashboard: ${path.relative(process.cwd(), DASHBOARD_HTML) || DASHBOARD_HTML}`);
}

main().catch((err) => {
  console.error(`\nERROR: ${err.message}`);
  process.exit(1);
});
