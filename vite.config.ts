import { resolve } from 'node:path'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { internalIpV4 } from 'internal-ip'

// eslint-disable-next-line node/prefer-global/process
const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM as string)

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: '0.0.0.0',
    hmr: mobile
      ? {
          protocol: 'ws',
          host: await internalIpV4(),
          port: 1420,
        }
      : undefined,
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
}))
