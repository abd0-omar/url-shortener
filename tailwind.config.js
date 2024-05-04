/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./templates/*.html'], // Corrected content property
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}
