import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        void: "#0B0C10",
        surface: {
          dark: "#14171E",
          elevated: "#1B202B",
          border: "#293142",
        },
        neon: {
          cyan: "#00F0FF",
          amber: "#FFB800",
          magenta: "#FF007A",
          green: "#00FF85",
          purple: "#9D00FF",
        },
      },
      fontFamily: {
        mono: ["JetBrains Mono", "Courier New", "monospace"],
      },
      boxShadow: {
        "neon-cyan": "0 0 15px rgba(0, 240, 255, 0.4)",
        "neon-magenta": "0 0 15px rgba(255, 0, 122, 0.4)",
        "neon-amber": "0 0 15px rgba(255, 184, 0, 0.4)",
      },
    },
  },
  plugins: [],
};
export default config;
