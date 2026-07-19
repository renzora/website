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
// Down-scaled widths for the feature-row and gallery previews, referenced via
// srcset so the browser fetches a size that matches its slot instead of the
// full-resolution source. Must stay in sync with the srcset widths in home.rs.
const PREVIEW_WIDTHS = [640, 1024];

// The .webp base for each preview is the canonical display master committed to
// git (some, like inspector, are hand-cropped and cannot be re-derived from the
// PNG). We generate the .avif companion and the responsive srcset variants from
// that .webp so any crop is preserved; the hero additionally derives wide
// variants from its high-res PNG below.
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
  const width = (await sharp(webp).metadata()).width;
  const avif = path.join(DIR, `${base}.avif`);
  await sharp(webp).avif({ quality: 52 }).toFile(avif);
  const w = fs.statSync(webp).size;
  const a = fs.statSync(avif).size;
  saved += w - a;
  console.log(`${base.padEnd(16)} webp ${(w / 1024).toFixed(0)}K → avif ${(a / 1024).toFixed(0)}K`);

  // Responsive down-scaled variants (webp + avif). Only emit widths smaller
  // than the source so we never upscale; the hero has its own wider set below.
  if (base !== HERO) {
    for (const vw of PREVIEW_WIDTHS) {
      if (vw >= width) continue;
      await sharp(webp).resize({ width: vw }).webp({ quality: 76 }).toFile(path.join(DIR, `${base}-${vw}.webp`));
      await sharp(webp).resize({ width: vw }).avif({ quality: 50 }).toFile(path.join(DIR, `${base}-${vw}.avif`));
    }
  }
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
