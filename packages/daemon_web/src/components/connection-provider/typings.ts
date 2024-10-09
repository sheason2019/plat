export enum ConnectionStatus {
  Pending,
  Open,
  Close,
}

export interface IDaemon {
  public_key: string;
  plugins: IPlugin[];
}

export interface IPlugin {
  name: string;
  wasm_root: string;
  assets_root: string;
  storage_root: string;
  entries: IPluginEntry[];
  address?: string;
}

export interface IPluginEntry {
  href: string;
  icon: string;
  label: string;
  target: string;
}

export interface IConnectionContext {
  ws?: WebSocket;
  daemon?: IDaemon;
}
