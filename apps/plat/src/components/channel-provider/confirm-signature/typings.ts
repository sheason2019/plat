export interface ConfirmSignatureData {
  data: string;
  describe: string;
  plugin_name: string;
  public_key: string;
}

export interface ConfirmSignatureAtom {
  id: string;
  data: ConfirmSignatureData;
}
