// Build a subset of the Phosphor icon font containing ONLY the icons this app
// uses. Scans the Rust source and SQL migrations for `ph-<name>` tokens (covers
// static markup, inline-JS class strings, and DB-seeded badge/category icons),
// then emits:
//   assets/fonts/phosphor-regular.woff2   (regular weight, subset)
//   assets/fonts/phosphor-fill.woff2      (fill weight, subset)
//   assets/style/phosphor.css             (@font-face + only the used icon rules)
//
// Run with `npm run build:icons`. Adding a new icon (in source or seed data)
// requires re-running this so the glyph is included.
import fs from 'fs';
import path from 'path';
import subsetFont from 'subset-font';

const ROOT = process.cwd();
const PH = path.join(ROOT, 'node_modules/@phosphor-icons/web/src');
const WEIGHTS = new Set(['ph-fill', 'ph-bold', 'ph-duotone', 'ph-thin', 'ph-light']);

// 1. Map icon name -> codepoint (hex) from the regular stylesheet
const css = fs.readFileSync(path.join(PH, 'regular/style.css'), 'utf8');
const map = {};
const ruleRe = /\.ph\.(ph-[a-z0-9-]+):before\s*\{\s*content:\s*"\\([0-9a-fA-F]+)"/g;
let r;
while ((r = ruleRe.exec(css))) map[r[1]] = r[2].toLowerCase();

// 2. Collect used icons SEPARATED BY WEIGHT so each font carries only its own
//    glyphs. We scan runs of class-like tokens: an icon in a run that also
//    contains `ph-fill` is used as fill; otherwise regular (covers `ph ph-x`,
//    bare `ph-x` seed tokens, and DB icons rendered with the `ph` prefix).
//    Conditional class strings like `${cond?'ph-fill ph-x':'ph ph-x'}` split
//    into separate runs, so `ph-x` correctly lands in both sets.
const regularUsed = new Set();
const fillUsed = new Set();
const allSeen = new Set(); // every ph-* token, for the "not in Phosphor" report
const runRe = /[A-Za-z0-9:_/[\].%#!-]+(?:[ \t]+[A-Za-z0-9:_/[\].%#!-]+)*/g;
function scan(text) {
  let run;
  while ((run = runRe.exec(text))) {
    const toks = run[0].split(/[ \t]+/);
    if (!toks.some((t) => t === 'ph' || t.startsWith('ph-'))) continue;
    const isFill = toks.includes('ph-fill');
    for (const t of toks) {
      if (t === 'ph' || WEIGHTS.has(t)) continue;
      if (/^ph-[a-z0-9-]+$/.test(t)) {
        allSeen.add(t);
        (isFill ? fillUsed : regularUsed).add(t);
      }
    }
  }
}
function walk(dir, exts) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, exts);
    else if (exts.includes(path.extname(e.name))) scan(fs.readFileSync(p, 'utf8'));
  }
}
walk(path.join(ROOT, 'crates'), ['.rs']);
walk(path.join(ROOT, 'migrations'), ['.sql']);

// 3. Keep only icons that exist in Phosphor, per weight
const regIcons = [...regularUsed].filter((n) => map[n]).sort();
const fillIcons = [...fillUsed].filter((n) => map[n]).sort();
const notFound = [...allSeen].filter((n) => !map[n]);
const toChars = (list) => list.map((n) => String.fromCodePoint(parseInt(map[n], 16))).join('');

// 4. Subset each weight to ONLY the glyphs used in that weight
async function subset(srcRel, outRel, chars) {
  const buf = fs.readFileSync(path.join(PH, srcRel));
  const out = await subsetFont(buf, chars, { targetFormat: 'woff2' });
  const dest = path.join(ROOT, outRel);
  fs.mkdirSync(path.dirname(dest), { recursive: true });
  fs.writeFileSync(dest, out);
  return out.length;
}
const fillWoff2 = fs.readdirSync(path.join(PH, 'fill')).find((f) => f.endsWith('.woff2'));
const rSize = await subset('regular/Phosphor.woff2', 'assets/fonts/phosphor-regular.woff2', toChars(regIcons));
// Only emit the fill weight if the app actually uses `ph-fill` icons — otherwise
// skip the whole font (no woff2, no @font-face, no .ph-fill rules).
const fSize = fillIcons.length
  ? await subset('fill/' + fillWoff2, 'assets/fonts/phosphor-fill.woff2', toChars(fillIcons))
  : 0;
if (!fillIcons.length) fs.rmSync('assets/fonts/phosphor-fill.woff2', { force: true });

// 5. Emit a minimal stylesheet (font-display:block avoids a tofu flash — the
//    subset is tiny and same-origin, so the block period is imperceptible).
//    Only the weight actually used gets a content rule for each icon.
const base =
  'speak:never;font-style:normal;font-weight:normal;font-variant:normal;text-transform:none;line-height:1;-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale';
// Emitted minified (single line, no whitespace between rules)
let out = '';
out += '@font-face{font-family:"Phosphor";src:url("/assets/fonts/phosphor-regular.woff2") format("woff2");font-weight:normal;font-style:normal;font-display:swap}';
if (fillIcons.length) out += '@font-face{font-family:"Phosphor-Fill";src:url("/assets/fonts/phosphor-fill.woff2") format("woff2");font-weight:normal;font-style:normal;font-display:swap}';
out += `.ph{font-family:"Phosphor"!important;${base}}`;
if (fillIcons.length) out += `.ph-fill{font-family:"Phosphor-Fill"!important;${base}}`;
// The closing quote delimits each \\XXXX hex escape, so no trailing space needed
for (const n of regIcons) out += `.ph.${n}:before{content:"\\${map[n]}"}`;
for (const n of fillIcons) out += `.ph-fill.${n}:before{content:"\\${map[n]}"}`;
fs.writeFileSync(path.join(ROOT, 'assets/style/phosphor.css'), out);

console.log(`regular icons: ${regIcons.length} | fill icons: ${fillIcons.length} | not in Phosphor: ${notFound.length}`);
if (notFound.length) console.log('  (skipped, not real Phosphor icons):', notFound.join(', '));
console.log(
  `regular: ${(rSize / 1024).toFixed(1)} KB  fill: ${(fSize / 1024).toFixed(1)} KB  css: ${(fs.statSync(path.join(ROOT, 'assets/style/phosphor.css')).size / 1024).toFixed(1)} KB`,
);
