export interface DaemonScope {
  local_daemons: Daemon[],
}

export interface Daemon {
  public_key: string;
  address: string;
  password: string;
}
