/** @type {import('tailwindcss').Config} */
import { Config } from "tailwindcss"

const tailwindConfig: Config = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx,vue}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
export default tailwindConfig