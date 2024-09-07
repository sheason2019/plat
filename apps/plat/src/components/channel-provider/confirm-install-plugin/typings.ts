import { PluginConfig } from "../../../models/core";

export interface ConfirmInstallPluginAtom {
  id: string;
  data: {
    public_key: string;
    plugin: PluginConfig;
  };
}
