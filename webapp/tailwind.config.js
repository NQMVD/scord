const { fontFamily } = require("tailwindcss/defaultTheme");

module.exports = {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["PP Supply Sans", "Inter var", ...fontFamily.sans],
      },
      borderRadius: {
        DEFAULT: "8px",
        secondary: "4px",
        container: "12px",
      },
      boxShadow: {
        DEFAULT: "0 2px 8px rgba(0, 0, 0, 0.4), 0 1px 2px rgba(0, 0, 0, 0.6)",
        hover: "0 4px 16px rgba(0, 0, 0, 0.5), 0 2px 4px rgba(0, 0, 0, 0.7)",
        glow: "0 0 20px rgba(109, 109, 109, 0.3), 0 0 40px rgba(109, 109, 109, 0.1)",
        "glow-hover":
          "0 0 30px rgba(136, 136, 136, 0.4), 0 0 60px rgba(136, 136, 136, 0.2)",
        inner: "inset 0 2px 4px rgba(0, 0, 0, 0.3)",
        "inner-glow":
          "inset 0 1px 0 rgba(255, 255, 255, 0.1), 0 1px 0 rgba(255, 255, 255, 0.05)",
        charcoal:
          "0 4px 12px rgba(13, 13, 13, 0.9), 0 2px 4px rgba(0, 0, 0, 0.95)",
        "charcoal-hover":
          "0 6px 20px rgba(10, 10, 10, 0.95), 0 4px 8px rgba(0, 0, 0, 0.98)",
      },
      colors: {
        charcoal: {
          50: "#f8f8f8",
          100: "#e8e8e8",
          200: "#d1d1d1",
          300: "#b0b0b0",
          400: "#888888",
          500: "#6d6d6d",
          600: "#5d5d5d",
          700: "#3f3f3f",
          800: "#2d2d2d",
          900: "#191919",
          950: "#0d0d0d",
          975: "#0a0a0a",
        },
        primary: {
          DEFAULT: "#191919",
          hover: "#2d2d2d",
          glow: "#3f3f3f",
        },
        secondary: {
          DEFAULT: "#2d2d2d",
          hover: "#3f3f3f",
          glow: "#5d5d5d",
        },
        accent: {
          DEFAULT: "#3f3f3f",
          hover: "#5d5d5d",
          glow: "#6d6d6d",
        },
      },
      spacing: {
        "form-field": "16px",
        section: "32px",
      },
    },
  },
  variants: {
    extend: {
      boxShadow: ["hover", "active"],
    },
  },
};
