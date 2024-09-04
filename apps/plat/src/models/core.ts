export interface Profile {
  daemons: Daemon[];
}

export interface Daemon {
  public_key: string;
  daemon_address: String;
  registed_plugins: Record<string, RegistedPlugin>;
}

export interface RegistedPlugin {
  addr: string;
  config: PluginConfig;
}

export interface PluginConfig {
  name: string;
  main: string;
  entries: PluginEntry[];
}

export interface PluginEntry {
  label: string;
  icon: string;
  href: string;
  target: string;
}
