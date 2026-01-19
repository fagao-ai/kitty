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
        // Primary brand colors
        primary: {
          DEFAULT: '#5352ed',
          hover: '#4544d4',
          active: '#3b3ab8',
          light: '#e8e8fe',
        },
        // Semantic colors
        success: '#10b981',
        warning: '#f59e0b',
        error: '#ef4444',
        info: '#3b82f6',

        // Accent color for special emphasis (e.g., speed test button)
        accent: {
          DEFAULT: '#63E2B7',
          light: '#d1fae5',
          dark: '#34d399',
        },

        // Background colors - Light mode
        bg: {
          DEFAULT: '#ffffff',
          muted: '#f8fafc',
          card: '#f1f5f9',
        },

        // Background colors - Dark mode
        'dark-bg': {
          DEFAULT: '#1e293b',
          muted: '#0f172a',
          card: '#334155',
        },

        // Text colors - Light mode (WCAG AA compliant)
        'text-primary': '#0f172a',
        'text-secondary': '#475569',
        'text-muted': '#64748b',

        // Legacy color name (for backward compatibility)
        primay: '#5352ed',
        dark: '#1e293b',
      },
      borderRadius: {
        'sm': '6px',
        'md': '8px',
        'lg': '12px',
        'xl': '16px',
        '2xl': '20px',
        'full': '9999px',
      },
      boxShadow: {
        'sm': '0 1px 2px 0 rgb(0 0 0 / 0.05)',
        'DEFAULT': '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)',
        'md': '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
        'lg': '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
        'xl': '0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)',
        '2xl': '0 25px 50px -12px rgb(0 0 0 / 0.25)',
        'inner': 'inset 0 2px 4px 0 rgb(0 0 0 / 0.05)',
        // Card specific shadows
        'card': '0 2px 8px rgba(0, 0, 0, 0.08), 0 1px 3px rgba(0, 0, 0, 0.05)',
        'card-hover': '0 8px 16px rgba(0, 0, 0, 0.12), 0 4px 8px rgba(0, 0, 0, 0.08)',
        'card-active': '0 0 0 2px #5352ed, 0 8px 16px rgba(83, 82, 237, 0.2)',
      },
      spacing: {
        '18': '4.5rem',
        '88': '22rem',
        '128': '32rem',
      },
      screens: {
        'sm': '640px',
        'md': '768px',
        'lg': '1024px',
        'xl': '1200px',
        'xxl': '1400px',
        'xxxl': '1500px',
        'tv': '1700px',
      },
      transitionDuration: {
        '250': '250ms',
        '350': '350ms',
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'shimmer': 'shimmer 2s linear infinite',
      },
      keyframes: {
        shimmer: {
          '0%': { backgroundPosition: '-1000px 0' },
          '100%': { backgroundPosition: '1000px 0' },
        },
      },
    },
  },
  plugins: [],
}
export default tailwindConfig
