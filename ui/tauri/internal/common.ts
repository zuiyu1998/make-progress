import { type UserConfig } from 'vite';

const commonConfig: UserConfig = {
  esbuild: {
    drop: ['debugger'],
  },
  build: {
    reportCompressedSize: false,
    chunkSizeWarningLimit: 1500,
    rollupOptions: {
      // TODO: Prevent memory overflow
      maxParallelFileOps: 3,
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
};

export { commonConfig };
