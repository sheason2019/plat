export enum ConnectionStatus {
  Pending,
  Open,
  Close,
}

export interface IDaemon {
  public_key: string;
}

export interface IConnectionContext {
  daemon: IDaemon;
}
