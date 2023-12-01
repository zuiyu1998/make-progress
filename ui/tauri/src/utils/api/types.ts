export interface ApiInstance {
  invoke<T>(cmd: string, record?: Record<string, unknown>): Promise<T>;
}

export class DefApi {
  apiInstance: ApiInstance;

  constructor(apiInstance: ApiInstance) {
    this.apiInstance = apiInstance;
  }

  invoke<T>(cmd: string, record?: Record<string, unknown>): Promise<T> {
    return this.apiInstance.invoke(cmd, record);
  }
}
