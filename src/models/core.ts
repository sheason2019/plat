export interface Profile {
  isolates: Isolate[];
}

export interface Isolate {
  public_key: string;
  private_key: string;
}
