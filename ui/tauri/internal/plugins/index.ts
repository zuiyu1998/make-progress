import react from '@vitejs/plugin-react';
import { PluginOption } from 'vite';
import { configHtmlPlugin } from './html';

interface Options {
  isBuild: boolean;
  root: string;
  compress: string;
  enableMock?: boolean;
  enableAnalyze?: boolean;
}

async function createPlugins({
  isBuild,
  root,
  enableMock,
  compress,
  enableAnalyze,
}: Options) {
  const vitePlugins: (PluginOption | PluginOption[])[] = [react()];

  // vite-plugin-html
  vitePlugins.push(await configHtmlPlugin({ isBuild }));

  // The following plugins only work in the production environment
  //   if (isBuild) {
  //     // rollup-plugin-gzip
  //     vitePlugins.push(
  //       configCompressPlugin({
  //         compress,
  //       })
  //     );
  //   }

  // rollup-plugin-visualizer
  //   if (enableAnalyze) {
  //     vitePlugins.push(configVisualizerConfig());
  //   }

  return vitePlugins;
}

export { createPlugins };
