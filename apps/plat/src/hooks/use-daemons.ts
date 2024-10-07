import { invoke } from "@tauri-apps/api/core";
import useSWR from "swr";
import { DaemonScope } from "../models/core";

export default function useDaemons() {
  const { data, mutate } = useSWR(
    "get_daemons",
    async (): Promise<DaemonScope> => {
      return JSON.parse(await invoke("get_daemons"));
    },
    { suspense: true }
  );

  return { daemons: data, mutate };
}
