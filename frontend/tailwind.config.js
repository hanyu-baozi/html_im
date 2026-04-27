/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#FF6B9D',
          light: '#FF8FB3',
          dark: '#E55A8A',
        },
        secondary: '#FFE4E1',
        accent: '#FFB6C1',
        'bg-primary': '#FFF5F7',
        success: '#98FB98',
        warning: '#FFD700',
        error: '#FF6B6B',
      },
      borderRadius: {
        'sm': '8px',
        'md': '16px',
        'lg': '24px',
        'xl': '32px',
      },
      boxShadow: {
        'sm': '0 2px 8px rgba(255, 107, 157, 0.1)',
        'md': '0 4px 16px rgba(255, 107, 157, 0.15)',
        'lg': '0 8px 32px rgba(255, 107, 157, 0.2)',
      },
    },
  },
  plugins: [],
}
