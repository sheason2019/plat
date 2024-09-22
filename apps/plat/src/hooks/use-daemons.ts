import { invoke } from "@tauri-apps/api/core";
import useSWR from "swr";
import { DaemonScope, DaemonVariant } from "../models/core";

export default function useDaemonScopes() {
  const { data, mutate } = useSWR(
    "get_daemons",
    async (): Promise<DaemonScope[]> => {
      return JSON.parse(await invoke("get_daemons"));
    },
    { suspense: true }
  );

  const getDaemonKey = (scope: DaemonScope) => {
    switch (scope.daemon.variant) {
      case DaemonVariant.Local:
        return encodeURIComponent(scope.daemon.public_key);
      default:
        return encodeURIComponent(scope.daemon.address);
    }
  };

  const findByDaemonKey = (daemonKey: string) => {
    return data.find(
      (scope) =>
        scope.daemon.address === daemonKey ||
        scope.daemon.public_key === daemonKey
    );
  };

  return { scopes: data, mutate, getDaemonKey, findByDaemonKey };
}
