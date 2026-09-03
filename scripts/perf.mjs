#!/usr/bin/env node
// ─────────────────────────────────────────────────────────────────────────────
// Performance measurement harness for renzora.com
//
// Pulls Lighthouse (lab) + CrUX (real-user field) metrics from the PageSpeed
// Insights API for a set of key routes, on both mobile and desktop. Prints a
// table and saves a timestamped JSON snapshot to perf-results/ so runs can be
// diffed before/after a change — this is how we PROVE "faster", not assert it.
//
// No local Chrome needed: PSI runs Lighthouse on Google's infra against the
// public URL — the exact source as the PageSpeed reports.
//
// Usage:
//   node scripts/perf.mjs                       # measure https://renzora.com (PSI)
//   node scripts/perf.mjs --local               # measure a running local build (localhost:3000)
//   node scripts/perf.mjs --base https://x.com  # measure another origin
//   node scripts/perf.mjs --routes /,/marketplace  # only these routes
//   node scripts/perf.mjs --compare             # diff the two latest snapshots
//   PAGESPEED_API_KEY=xxx node scripts/perf.mjs # raise PSI rate limits
//
// --local runs Lighthouse on your machine via headless Chrome (no API key, no
// public URL needed) — ideal for checking a build before you push it. It needs
// the `lighthouse` + `chrome-launcher` dev-deps (npm install) and a local Chrome.
//
// A free API key (higher limits, avoids 429s) comes from any Google Cloud
// project with the "PageSpeed Insights API" enabled.
// ─────────────────────────────────────────────────────────────────────────────

import fs from 'fs';
import path from 'path';

const ROOT = process.cwd();
const OUT_DIR = path.join(ROOT, 'perf-results');
const API = 'https://www.googleapis.com/pagespeedonline/v5/runPagespeed';
const KEY = process.env.PAGESPEED_API_KEY || '';

const DEFAULT_ROUTES = ['/', '/marketplace', '/docs'];
const STRATEGIES = ['mobile', 'desktop'];

// ── args ──
const argv = process.argv.slice(2);
const arg = (name, def) => {
  const i = argv.indexOf(name);
  return i >= 0 && argv[i + 1] ? argv[i + 1] : def;
};
const COMPARE = argv.includes('--compare');
const LOCAL = argv.includes('--local');
const BASE = arg('--base', LOCAL ? 'http://localhost:3000' : 'https://renzora.com').replace(/\/$/, '');
const ROUTES = arg('--routes', '').split(',').filter(Boolean).length
  ? arg('--routes', '').split(',').filter(Boolean)
  : DEFAULT_ROUTES;

// ── metric extractors ──
const audit = (r, id) => r.lighthouseResult?.audits?.[id]?.numericValue ?? null;
const lab = (r) => ({
  perf: Math.round((r.lighthouseResult?.categories?.performance?.score ?? 0) * 100),
  ttfb: audit(r, 'server-response-time'),
  fcp: audit(r, 'first-contentful-paint'),
  lcp: audit(r, 'largest-contentful-paint'),
  tbt: audit(r, 'total-blocking-time'),
  cls: audit(r, 'cumulative-layout-shift'),
  si: audit(r, 'speed-index'),
  bytes: audit(r, 'total-byte-weight'),
});
// CrUX field data (real users, 28-day p75). Present only for URLs/origins with
// enough traffic; falls back to origin-level when the exact URL has too little.
const field = (r) => {
  const src = r.loadingExperience?.metrics ? r.loadingExperience : r.originLoadingExperience;
  const m = src?.metrics;
  if (!m) return null;
  const p = (k) => m[k]?.percentile ?? null;
  return {
    scope: r.loadingExperience?.metrics ? 'url' : 'origin',
    lcp: p('LARGEST_CONTENTFUL_PAINT_MS'),
    inp: p('INTERACTION_TO_NEXT_PAINT'),
    cls: p('CUMULATIVE_LAYOUT_SHIFT_SCORE'),
    fcp: p('FIRST_CONTENTFUL_PAINT_MS'),
    ttfb: p('EXPERIMENTAL_TIME_TO_FIRST_BYTE'),
  };
};

// ── formatting ──
const t = (v) => (v == null ? '—' : v >= 1000 ? (v / 1000).toFixed(2) + 's' : Math.round(v) + 'ms');
const c = (v) => (v == null ? '—' : (v > 1 ? v / 100 : v).toFixed(3)); // CrUX CLS is ×100
const kb = (v) => (v == null ? '—' : Math.round(v / 1024) + 'KB');
const pad = (s, n) => String(s).padEnd(n);

async function psi(url, strategy) {
  const params = new URLSearchParams({ url, strategy, category: 'performance' });
  if (KEY) params.set('key', KEY);
  for (let attempt = 1; attempt <= 4; attempt++) {
    let res;
    try {
      res = await fetch(`${API}?${params}`);
    } catch (e) {
      if (attempt === 4) throw e;
      await sleep(attempt * 3000);
      continue;
    }
    if (res.ok) return res.json();
    if (res.status === 429 || res.status >= 500) {
      const wait = attempt * 5000;
      process.stderr.write(`  ${strategy} ${url} → ${res.status}, retry in ${wait / 1000}s\n`);
      await sleep(wait);
      continue;
    }
    throw new Error(`PSI ${res.status} for ${strategy} ${url}: ${(await res.text()).slice(0, 160)}`);
  }
  throw new Error(`PSI failed after retries: ${strategy} ${url}`);
}
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

// ── backends: PSI (remote, adds CrUX field data) or local Lighthouse (any URL,
//    no API key — measure a build before you deploy) ──
function psiRunner() {
  return { mode: 'psi', run: (url, strategy) => psi(url, strategy), cleanup: async () => {} };
}

async function localRunner() {
  let lighthouse, chromeLauncher, desktopConfig;
  try {
    ({ default: lighthouse } = await import('lighthouse'));
    chromeLauncher = await import('chrome-launcher');
    ({ default: desktopConfig } = await import('lighthouse/core/config/desktop-config.js'));
  } catch (e) {
    throw new Error('Local mode needs the lighthouse dev-deps — run `npm install` first.\n  ' + e.message);
  }
  const chrome = await chromeLauncher.launch({ chromeFlags: ['--headless=new', '--no-sandbox', '--disable-gpu'] });
  return {
    mode: 'local',
    // Mobile is Lighthouse's default (simulated slow-4G + 4× CPU throttle); desktop
    // uses the built-in desktop preset. Result shape mirrors PSI's `lighthouseResult`
    // so the same extractors, snapshots and `--compare` work across both backends.
    run: async (url, strategy) => {
      const flags = { port: chrome.port, onlyCategories: ['performance'], logLevel: 'silent' };
      const result = await lighthouse(url, flags, strategy === 'desktop' ? desktopConfig : undefined);
      if (!result?.lhr) throw new Error('Lighthouse returned no result (is the server up at ' + url + '?)');
      return { lighthouseResult: result.lhr };
    },
    cleanup: () => chrome.kill(),
  };
}

async function measure() {
  const { run, cleanup, mode } = LOCAL ? await localRunner() : psiRunner();
  console.log(`Measuring ${BASE} via ${mode === 'local' ? 'local Lighthouse' : 'PageSpeed Insights'}${KEY ? ' (keyed)' : ''}\n`);
  const runs = [];
  try {
    for (const strategy of STRATEGIES) {
      for (const route of ROUTES) {
        const url = BASE + route;
        process.stdout.write(`· ${pad(strategy, 7)} ${route} … `);
        try {
          const r = await run(url, strategy);
          const row = { route, strategy, lab: lab(r), field: field(r) };
          runs.push(row);
          process.stdout.write(`perf ${row.lab.perf}  LCP ${t(row.lab.lcp)}  TBT ${t(row.lab.tbt)}\n`);
        } catch (e) {
          process.stdout.write(`FAILED (${e.message})\n`);
          runs.push({ route, strategy, error: e.message });
        }
      }
    }
  } finally {
    // Chrome's temp-profile removal can throw EPERM on Windows (the files are
    // still locked as the process exits). It's harmless and must never discard
    // the results we just collected.
    try {
      await cleanup();
    } catch (e) {
      process.stderr.write(`(ignored Chrome cleanup error: ${e.code || e.message})\n`);
    }
  }

  const snapshot = { timestamp: new Date().toISOString(), mode, base: BASE, runs };
  fs.mkdirSync(OUT_DIR, { recursive: true });
  const file = path.join(OUT_DIR, snapshot.timestamp.replace(/[:.]/g, '-') + '.json');
  fs.writeFileSync(file, JSON.stringify(snapshot, null, 2));

  printTables(snapshot);
  console.log(`\nSaved → ${path.relative(ROOT, file)}`);
  console.log('Diff against the previous run any time with:  node scripts/perf.mjs --compare');
}

function printTables(snap) {
  for (const strategy of STRATEGIES) {
    const rows = snap.runs.filter((r) => r.strategy === strategy && r.lab);
    if (!rows.length) continue;
    console.log(`\n── ${strategy.toUpperCase()} — lab (Lighthouse) ──`);
    console.log(
      `${pad('route', 14)} ${pad('perf', 5)} ${pad('TTFB', 7)} ${pad('FCP', 7)} ${pad('LCP', 7)} ${pad('TBT', 7)} ${pad('CLS', 6)} ${pad('SI', 7)} ${pad('bytes', 7)}`,
    );
    for (const r of rows) {
      const l = r.lab;
      console.log(
        `${pad(r.route, 14)} ${pad(l.perf, 5)} ${pad(t(l.ttfb), 7)} ${pad(t(l.fcp), 7)} ${pad(t(l.lcp), 7)} ${pad(t(l.tbt), 7)} ${pad(c(l.cls), 6)} ${pad(t(l.si), 7)} ${pad(kb(l.bytes), 7)}`,
      );
    }
    const fieldRows = rows.filter((r) => r.field);
    if (fieldRows.length) {
      console.log(`\n   real-user field data (CrUX p75):`);
      for (const r of fieldRows) {
        const f = r.field;
        console.log(
          `   ${pad(r.route + ` (${f.scope})`, 22)} LCP ${pad(t(f.lcp), 7)} INP ${pad(t(f.inp), 7)} CLS ${pad(c(f.cls), 6)} FCP ${pad(t(f.fcp), 7)} TTFB ${t(f.ttfb)}`,
        );
      }
    }
  }
}

function latestTwo() {
  if (!fs.existsSync(OUT_DIR)) return [];
  return fs
    .readdirSync(OUT_DIR)
    .filter((f) => f.endsWith('.json'))
    .sort()
    .slice(-2)
    .map((f) => JSON.parse(fs.readFileSync(path.join(OUT_DIR, f), 'utf8')));
}

function compare() {
  const snaps = latestTwo();
  if (snaps.length < 2) {
    console.error('Need at least two snapshots in perf-results/ to compare. Run `node scripts/perf.mjs` twice.');
    process.exit(1);
  }
  const [before, after] = snaps;
  console.log(`Comparing:\n  before  ${before.timestamp}\n  after   ${after.timestamp}\n`);
  if (before.base !== after.base || before.mode !== after.mode) {
    console.log(`⚠  Different sources — before: ${before.mode || '?'} ${before.base}, after: ${after.mode || '?'} ${after.base}. Deltas may not be comparable.\n`);
  }
  const key = (r) => `${r.strategy} ${r.route}`;
  const map = new Map(before.runs.map((r) => [key(r), r]));
  const delta = (a, b, fmt, lowerIsBetter = true) => {
    if (a == null || b == null) return '—';
    const d = b - a;
    if (Math.abs(d) < (fmt === c ? 0.001 : 1)) return `${fmt(b)} (=)`;
    const better = lowerIsBetter ? d < 0 : d > 0;
    const sign = d > 0 ? '+' : '';
    return `${fmt(b)} (${sign}${fmt === c ? d.toFixed(3) : fmt(Math.abs(d))} ${better ? '✓' : '✗'})`;
  };
  for (const strategy of STRATEGIES) {
    const rows = after.runs.filter((r) => r.strategy === strategy && r.lab);
    if (!rows.length) continue;
    console.log(`── ${strategy.toUpperCase()} ──`);
    console.log(`${pad('route', 14) } ${pad('perf', 12)} ${pad('LCP', 16)} ${pad('TBT', 16)} ${pad('CLS', 14)}`);
    for (const r of rows) {
      const p = map.get(key(r));
      if (!p?.lab) { console.log(`${pad(r.route, 14)} (no baseline)`); continue; }
      const perfD = r.lab.perf - p.lab.perf;
      const perfStr = `${r.lab.perf} (${perfD >= 0 ? '+' : ''}${perfD} ${perfD >= 0 ? '✓' : '✗'})`;
      console.log(
        `${pad(r.route, 14)} ${pad(perfStr, 12)} ${pad(delta(p.lab.lcp, r.lab.lcp, t), 16)} ${pad(delta(p.lab.tbt, r.lab.tbt, t), 16)} ${pad(delta(p.lab.cls, r.lab.cls, c), 14)}`,
      );
    }
    console.log('');
  }
}

if (COMPARE) compare();
else measure().catch((e) => { console.error(e); process.exit(1); });
