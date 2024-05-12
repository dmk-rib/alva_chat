/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}", 
    "./dist/**/*.html",
    // "node_modules/preline/dist/*.js"
  ],
  // enable dark mode via class strategy
  darkMode: 'class',
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    // require('preline/plugin')
  ],
};
