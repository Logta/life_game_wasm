import { defineConfig, type Plugin } from "vite";
import wasm from "vite-plugin-wasm";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = fileURLToPath(new URL('.', import.meta.url));

// 本番ビルドの index.html に埋め込む Content-Security-Policy。
// GitHub Pages ではレスポンスヘッダを設定できないため <meta http-equiv> で指定する。
// - script-src: 自オリジンのバンドルのみ。WASM のコンパイルに 'wasm-unsafe-eval' が必要
// - style-src: index.html のインライン <style> のため 'unsafe-inline' を許可（スクリプトは不可）
// - img-src: data: のファビコン
// - connect-src: .wasm の fetch（自オリジン）
// frame-ancestors は <meta> では無効なので指定しない。
const CONTENT_SECURITY_POLICY = [
  "default-src 'self'",
  "script-src 'self' 'wasm-unsafe-eval'",
  "style-src 'self' 'unsafe-inline'",
  "img-src 'self' data:",
  "connect-src 'self'",
  "object-src 'none'",
  "base-uri 'self'",
  "form-action 'none'",
].join("; ");

// 開発サーバー（HMR のインラインスクリプトや WebSocket を使う）には適用せず、ビルド時だけ CSP を注入する
function contentSecurityPolicy(): Plugin {
  return {
    name: "content-security-policy",
    apply: "build",
    transformIndexHtml() {
      return [
        {
          tag: "meta",
          attrs: { "http-equiv": "Content-Security-Policy", content: CONTENT_SECURITY_POLICY },
          injectTo: "head-prepend",
        },
      ];
    },
  };
}

// Vite configuration for the Life Game WASM project
// Life Game WASMプロジェクトのVite設定
export default defineConfig({
  plugins: [wasm(), contentSecurityPolicy()],
  
  // Base path for GitHub Pages deployment
  base: process.env.GITHUB_ACTIONS ? '/life_game_wasm/' : '/',
  
  // Server configuration
  server: {
    port: 3000,
    open: true,
    fs: {
      // Restrict file system access for security
      // "." がプロジェクトルート全体(pkg/を含む)を許可するため、
      // 個別に "./pkg" を指定する必要はない
      strict: true,
      allow: ["."]
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