import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import { resolve } from "path";

// Vite configuration for the Life Game WASM project
// Life Game WASMプロジェクトのVite設定
export default defineConfig({
  plugins: [wasm()],
  
  // Server configuration
  server: {
    port: 3000,
    open: true,
    fs: {
      // Restrict file system access for security
      strict: true,
      allow: [".", "../pkg"]
    }
  },
  
  // Build configuration
  build: {
    target: "esnext",
    outDir: "dist",
    emptyOutDir: true,
    sourcemap: true,
    minify: "terser",
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true
      }
    },
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html")
      },
      output: {
        assetFileNames: "assets/[name]-[hash][extname]",
        chunkFileNames: "assets/[name]-[hash].js",
        entryFileNames: "assets/[name]-[hash].js"
      }
    }
  },
  
  // Optimization options
  optimizeDeps: {
    exclude: ["life_game_wasm"]
  },
  
  // Resolve aliases
  resolve: {
    alias: {
      "@": resolve(__dirname, "./src"),
      "@wasm": resolve(__dirname, "./pkg")
    }
  }
});