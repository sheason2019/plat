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
  address: string;
  assets_root: string;
  entries: IPluginEntry[];
  name: string;
  storage_root: string;
  version: string;
  wasm_root: string;
  daemon_address?: string;
  regist_address?: string;
}

export interface IPluginEntry {
  href: string;
  icon: string;
  label: string;
  target: string;
}

export interface IConnectionContext {
  daemon: IDaemon;
}
