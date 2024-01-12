/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],  theme: {
    extend: {
      colors: {
        primary: "var(--primary)",
        secondary: "var(--secondary)",
        text: "var(--textSecondary)",
        textSecondary: "var(--textSecondary)",
        accent: "var(--accent)",
        border: "var(--border)",
        shadow: "var(--shadow)"
      }
    },
  },
  plugins: [],
}

