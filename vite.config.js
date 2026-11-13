import { defineConfig } from "vite";

export default defineConfig({
  worker: {
    format: "es"
  },
  optimizeDeps: {
    include: ["monaco-editor"]
  },
  build: {
    target: "es2022"
  }
});
