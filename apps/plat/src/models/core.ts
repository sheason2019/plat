export interface DaemonScope {
  daemon: Daemon;
}

export enum DaemonVariant {
  Local = "Local",
  Remote = "Remote",
  Hybrid = "Hybrid",
}

export interface Daemon {
  public_key: string;
  address: string;
  password: string;
}
