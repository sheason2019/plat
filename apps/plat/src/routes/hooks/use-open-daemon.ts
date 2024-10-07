import { invoke } from "@tauri-apps/api/core";
import { Daemon, RemoteDaemon } from "../../models/core";

export default function useOpenDaemon() {
  const openDaemon = async (daemon: Daemon | RemoteDaemon) => {
    const address = new URL(daemon.address);
    await invoke("open_daemon_window", { address });
  };

  return {
    openDaemon,
  };
}
