/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./components/*.{ts, tsx}","./app/*.{ts, tsx}","./app/**/*.{ts, tsx}"],
  presets: [require("nativewind/preset")],
  theme: {
    extend: {},
  },
  plugins: [],
}

