import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import Components from 'unplugin-vue-components/vite';
import { PrimeVueResolver } from '@primevue/auto-import-resolver';
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [vue(),
  Components({
    dirs: ["./src/pages", "./src/components", "./src/layouts"],
    resolvers: [
      PrimeVueResolver()
    ]
  })],

  clearScreen: false,
  server: {
    port: 1420,
    strictPort: false,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
