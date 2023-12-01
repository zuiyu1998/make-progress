import { ApiInstance } from './types';
import { invoke } from '@tauri-apps/api';

export class Client implements ApiInstance {
  constructor() {}

  invoke<T>(cmd: string, record: Record<string, unknown>): Promise<T> {
    return invoke<T>(cmd, record);
  }
}
