import { IPlugin } from "../connection-provider/typings";

export enum ConfirmModalVariant {
  InstallPlugin,
  RemovePlugin,
}

export type ConfirmModalState =
  | null
  | IConfirmInstallPluginState
  | IConfirmRemovePluginState;

export interface IConfirmInstallPluginState {
  variant: ConfirmModalVariant.InstallPlugin;
  name: string;
  plugin: IPlugin;
}

export interface IConfirmRemovePluginState {
  variant: ConfirmModalVariant.RemovePlugin;
  name: string;
}
