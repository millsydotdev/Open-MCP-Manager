/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{rs,html,css}"],
    theme: {
        extend: {
            colors: {
                'app-dark': '#09090b', // Matching our theme
                'app-secondary': '#111115',
            }
        },
    },
    plugins: [],
}
