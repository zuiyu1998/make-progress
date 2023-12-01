/**
 * Plugin to minimize and use ejs template syntax in index.html.
 * https://github.com/anncwb/vite-plugin-html
 */
import type { PluginOption } from 'vite';
import { createHtmlPlugin } from 'vite-plugin-html';
import { getEnvConfig } from '../utils/env';
import { log } from 'console';

export async function configHtmlPlugin({ isBuild }: { isBuild: boolean }) {
  const envConfig = await getEnvConfig();

  const htmlPlugin: PluginOption[] = createHtmlPlugin({
    minify: isBuild,
    inject: {
      data: {
        ...envConfig,
      },
    },
  });
  return htmlPlugin;
}
