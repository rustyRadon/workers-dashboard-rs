/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: [
      "*.html",
      "./src/**/*.rs", // This tells Tailwind to look inside your Rust files
    ],
  },
  theme: {
    extend: {},
  },
  plugins: [],
}