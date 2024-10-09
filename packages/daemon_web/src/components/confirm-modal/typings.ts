import { IPlugin } from "../connection-provider/typings";

export enum ConfirmModalVariant {
  InstallPlugin,
  DeletePlugin,
}

export type ConfirmModalState =
  | null
  | IConfirmInstallPluginState
  | IConfirmDeletePluginState;

export interface IConfirmInstallPluginState {
  variant: ConfirmModalVariant.InstallPlugin;
  name: string;
  plugin: IPlugin;
}

export interface IConfirmDeletePluginState {
  variant: ConfirmModalVariant.DeletePlugin;
  plugin: IPlugin;
}
