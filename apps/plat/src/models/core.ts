export interface DaemonScope {
  daemon: Daemon;
  plugins: Plugin[];
}

export enum DaemonVariant {
  Local = "Local",
  Remote = "Remote",
  Hybrid = "Hybrid",
}

export interface Daemon {
  public_key: string;
  private_key: string;
  password: string;
  variant: DaemonVariant;
  address: string;
}

export interface Plugin {
  name: string;
  main: string;
  entries: {
    label: string;
    icon: string;
    href: string;
    target: string;
  }[];
}
