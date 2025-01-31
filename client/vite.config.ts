import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    cors: {
      origin: "*",
    },
  },
  build: {
    manifest: true,
    rollupOptions: {
      // input: ["src/main.tsx"],
    },
    modulePreload: false,
    sourcemap: true,
  },
});
