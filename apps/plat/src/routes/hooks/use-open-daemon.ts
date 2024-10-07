import { invoke } from "@tauri-apps/api/core";
import { Daemon, RemoteDaemon } from "../../models/core";

export default function useOpenDaemon() {
  const openDaemon = async (daemon: Daemon | RemoteDaemon) => {
    const address = new URL(daemon.address);
    address.searchParams.set("fromOrigin", location.origin);
    address.searchParams.set("password", daemon.password);
    console.log("address", address.toString());

    await invoke("open_daemon_window", { address: address.toString() });
  };

  return {
    openDaemon,
  };
}
