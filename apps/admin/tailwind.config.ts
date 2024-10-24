import type { Config } from "tailwindcss"
import withMT from "@material-tailwind/react/utils/withMT"

const config: Config = {
  darkMode: "class",
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        inter: ["var(--font-inter)", "sans-serif"],
        helvetica: ["var(--font-helvetica)", "sans-serif"],
      },
      colors: {
        "primary": "var(--color-primary)",
        "secondary": "var(--color-secondary)",
        "white": "var(--color-white)",
        "background": "var(--color-background)",
        "primary-bg": "var(--color-primary-bg)",
        "warning": "var(--color-warning)",
        "warning-bg": "var(--color-warning-bg)",
        "success": "var(--color-success)",
        "success-bg": "var(--color-success-bg)",
        "error": "var(--color-error)",
        "error-bg": "var(--color-error-bg)",
        "grey": {
          0: "var(--color-grey-0)",
          2: "var(--color-grey-2)",
          4: "var(--color-grey-4)",
          5: "var(--color-grey-5)",
        },
      },
    },
  },
  plugins: [],
}

export default withMT(config)
