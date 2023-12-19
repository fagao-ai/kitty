/** @type {import('tailwindcss').Config} */
import type { Config } from 'tailwindcss'

const tailwindConfig: Config = {
  darkMode: 'class',
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx,vue}',
  ],
  theme: {
    extend: {
      colors: {
        primay: '#5352ed',
        dark: '#3e4247',
      },
      screens: {
        xl: '1200px',
        xxl: '1400px',
        xxxl: '1500px',
        tv: '1700px',
      },
    },
  },
  plugins: [],
}
export default tailwindConfig
