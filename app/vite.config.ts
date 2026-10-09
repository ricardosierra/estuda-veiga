import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    target: process.env.TAURI_ENV_PLATFORM === "darwin" ? "safari14" : "chrome105",
    minify: !process.env.TAURI_ENV_DEBUG ? "oxc" : false,
    sourcemap: Boolean(process.env.TAURI_ENV_DEBUG),
  },
});
