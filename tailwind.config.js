/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  // Scan the Leptos source (markup + inline JS template strings live in .rs files).
  // Every dynamic class (gradient stops, status colors) is written as a whole
  // literal token in these files — e.g. `color="from-orange-500 to-amber-500"`
  // and JS arrays of full class strings — so the scanner catches them all without
  // a safelist. (A broad safelist previously padded the output with ~200 unused
  // gradient utilities.)
  content: ['./crates/web/src/**/*.rs'],
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
