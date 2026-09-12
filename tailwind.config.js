/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  // Scan the Leptos source (markup + inline JS template strings live in .rs files).
  // Every dynamic class (gradient stops, status colors) is written as a whole
  // literal token in these files — e.g. `color="from-orange-500 to-amber-500"`
  // and JS arrays of full class strings — so the scanner catches them all without
  // a safelist. (A broad safelist previously padded the output with ~200 unused
  // gradient utilities.)
  // `assets/js` is scanned for the same reason: the stats chart builds its DOM in
  // a standalone script rather than in a .rs template, and without this glob every
  // utility it names is absent from the bundle and the chart renders unstyled.
  // The same rule applies there: whole literal tokens only, never a class built by
  // concatenating a computed value, which the scanner cannot see.
  content: ['./crates/web/src/**/*.rs', './assets/js/**/*.js'],
  theme: {
    extend: {
      colors: {
        accent: { DEFAULT: '#a855f7', hover: '#c084fc', subtle: 'rgba(168,85,247,0.12)' },
        secondary: { DEFAULT: '#22d3ee', hover: '#67e8f9' },
        surface: { DEFAULT: '#120b1f', card: '#160d26', panel: '#0c0715' },
      },
      fontFamily: {
        sans: ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
        mono: ['Cascadia Code', 'Fira Code', 'monospace'],
      },
    },
  },
  plugins: [],
};
