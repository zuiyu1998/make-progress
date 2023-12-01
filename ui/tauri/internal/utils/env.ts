import { join } from 'node:path';

import fsExtra from 'fs-extra';

const { readFile } = fsExtra;

/**
 * 获取当前环境下生效的配置文件名
 */
function getConfFiles() {
  return ['./src-tauri/tauri.conf.json'];
}

/**
 * Get the environment variables starting with the specified prefix
 * @param match prefix
 * @param confFiles ext
 */
export async function getEnvConfig(
  match = 'VITE_GLOB_',
  confFiles = getConfFiles()
) {
  let tmpConfig = {};

  for (const confFile of confFiles) {
    try {
      const envPath = await readFile(join(process.cwd(), confFile), {
        encoding: 'utf8',
      });
      const env = JSON.parse(envPath)?.package ?? {};
      tmpConfig = { ...tmpConfig, ...env };
    } catch (e) {
      console.error(`Error in parsing ${confFile}`, e);
    }
  }

  let envConfig = {};

  Object.keys(tmpConfig).forEach((key) => {
    envConfig[match + key.toUpperCase()] = tmpConfig[key];
  });

  return envConfig;
}
