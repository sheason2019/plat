import { invoke } from "@tauri-apps/api/core";
import useSWR from "swr";
import { Profile } from "../../models/core";

export default function useProfile() {
  return useSWR(
    "get_profile",
    async (): Promise<Profile> => {
      return JSON.parse(await invoke("get_profile"));
    },
    { suspense: true }
  );
}
