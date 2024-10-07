export interface DaemonScope {
  local_daemons: Daemon[];
  remote_daemons: RemoteDaemon[];
}

export interface Daemon {
  public_key: string;
  address: string;
  password: string;
}

export interface RemoteDaemon {
  address: string;
  password: string;
}
