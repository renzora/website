// Generate optimized image variants for assets/previews from the high-res PNG
// sources (the PNGs are the originals; the site serves the derived formats):
//   • every preview gets a same-size .avif next to its .webp (~30–50% smaller),
//     used via <picture> with the .webp as the fallback.
//   • the hero (interface) additionally gets responsive widths in .webp + .avif
//     so mobile loads a small image instead of the full-width one (LCP win).
//
// Run with `npm run optimize:images` after changing any source PNG. Output
// files live alongside the sources in assets/previews/.
import sharp from 'sharp';
import fs from 'fs';
import path from 'path';

const DIR = 'assets/previews';
const HERO = 'interface';
const HERO_WIDTHS = [640, 1280, 1920];

// The <picture> fallback for each preview stays .webp; we only add .avif here.
// Regenerate from the PNG source when present (best quality), else from the webp.
const isVariant = (f) => /-\d+\.(webp|avif)$/.test(f);
const bases = [
  ...new Set(
    fs
      .readdirSync(DIR)
      .filter((f) => f.endsWith('.webp') && !isVariant(f))
      .map((f) => f.replace('.webp', '')),
  ),
];

let saved = 0;
for (const base of bases) {
  const webp = path.join(DIR, `${base}.webp`);
  const png = path.join(DIR, `${base}.png`);
  const src = fs.existsSync(png) ? png : webp;
  const width = (await sharp(webp).metadata()).width;
  const avif = path.join(DIR, `${base}.avif`);
  await sharp(src).resize({ width }).avif({ quality: 52 }).toFile(avif);
  const w = fs.statSync(webp).size;
  const a = fs.statSync(avif).size;
  saved += w - a;
  console.log(`${base.padEnd(16)} webp ${(w / 1024).toFixed(0)}K → avif ${(a / 1024).toFixed(0)}K`);
}

// Hero: responsive widths from the high-res PNG (falls back to the webp).
const heroSrc = fs.existsSync(path.join(DIR, `${HERO}.png`))
  ? path.join(DIR, `${HERO}.png`)
  : path.join(DIR, `${HERO}.webp`);
console.log(`\nhero responsive variants from ${path.basename(heroSrc)}:`);
for (const w of HERO_WIDTHS) {
  const wp = path.join(DIR, `${HERO}-${w}.webp`);
  const av = path.join(DIR, `${HERO}-${w}.avif`);
  await sharp(heroSrc).resize({ width: w }).webp({ quality: 76 }).toFile(wp);
  await sharp(heroSrc).resize({ width: w }).avif({ quality: 50 }).toFile(av);
  console.log(`  ${w}w  webp ${(fs.statSync(wp).size / 1024).toFixed(0)}K  avif ${(fs.statSync(av).size / 1024).toFixed(0)}K`);
}

console.log(`\ntotal saved (webp→avif on base previews): ${(saved / 1024).toFixed(0)}K`);
