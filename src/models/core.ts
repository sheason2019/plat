export interface Profile {
  isolates: Isolate[];
}

export interface Isolate {
  public_key: string;
  private_key: string;
  plugins: Plugin[];
}

export interface Plugin {
  name: string;
  plugin: string;
  addr: string;
  entries: Entry[];
}

export interface Entry {
  name: string;
  href: string;
  target: string;
}
