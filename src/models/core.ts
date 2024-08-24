export interface Profile {
  isolates: Isolate[];
}

export interface Isolate {
  public_key: string;
  private_key: string;
  plugins: Record<string, RegistedPlugin>;
}

export interface RegistedPlugin {
  addr: string;
  config: PlatXConfig;
}

export interface PlatXConfig {
  name: string;
  main: string;
  entries: Entry[];
}

export interface Entry {
  label: string;
  icon: string;
  href: string;
  target: string;
}
