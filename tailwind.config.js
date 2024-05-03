/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
          peach: {
            50: '#fdf2f6',
            100: '#fbe8f0',
            200: '#fad0e1',
            300: '#f6abc8',
            400: '#f076a2',
            500: '#e6497d',
            600: '#d52d5d',
            700: '#b91d45',
            800: '#991b3a',
            900: '#7f1c34',
            950: '#4e0919',
        }
      }
    }
  },
  plugins: [require('flowbite/plugin')],
}