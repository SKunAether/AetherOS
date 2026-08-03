// Vite 构建工具配置：定义构建规则与 Tauri 开发服务器适配
// 注意：Tauri 要求固定端口与 strictPort，且需关闭 HMR 的 host 检查。
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'node:path';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [react()],

  // Tauri 固定端口，避免开发时 WebView 加载地址漂移
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 忽略 src-tauri 下的 Rust 源码变更，避免无谓的重载
      ignored: ['**/src-tauri/**'],
    },
  },

  // 路径别名 @ -> src，配合 tsconfig
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },

  // 构建产物输出到 src-tauri 目标目录（Tauri 打包约定）
  build: {
    target: 'es2021',
    minify: 'esbuild',
    sourcemap: Boolean(process.env.TAURI_ENV_DEBUG),
  },
});
