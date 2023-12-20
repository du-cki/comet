/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      color: {
        'nav-bg': '#111111',
      },
    },
  },
  plugins: [],
}

