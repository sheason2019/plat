import { PropsWithChildren, useEffect, useState } from "react";
import ConnectionPending from "./pending";
import { ConnectionStatus, IDaemon } from "./typings";
import ConnectionClose from "./close";
import { useSetRecoilState } from "recoil";
import { connectionState } from "./context";
import useConfirmModal from "../confirm-modal/hooks/use-confirm-modal";
import useOrigin from "../../hooks/use-origin";

export default function ConnectionProvider({ children }: PropsWithChildren) {
  const [status, setStatus] = useState(ConnectionStatus.Pending);
  const [closeReason, setCloseReason] = useState("");
  const { confirmInstallPlugin, confirmDeletePlugin } = useConfirmModal();

  const setConnection = useSetRecoilState(connectionState);

  const connectionOrigin: string = useOrigin().replace("http", "ws");

  useEffect(() => {
    setStatus(ConnectionStatus.Pending);

    const origin = connectionOrigin.endsWith("/")
      ? connectionOrigin.slice(0, -1)
      : connectionOrigin;
    const ws = new WebSocket(`${origin}/api/connect`);

    const handleMessage = async (message: string) => {
      const data: {
        type: string;
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        payload: any;
      } = JSON.parse(message);
      console.info("receive message", data);
      switch (data.type) {
        case "ok":
          setStatus(ConnectionStatus.Open);
          break;
        case "daemon":
          setConnection((prev) => ({
            ...prev,
            daemon: data.payload as IDaemon,
          }));
          break;
        case "confirm/install-plugin":
          confirmInstallPlugin(data.payload.name, data.payload.plugin);
          break;
        case "confirm/delete-plugin":
          confirmDeletePlugin(data.payload.plugin);
          break;
        default:
          break;
      }
    };

    const messageListener = async (e: MessageEvent) => {
      handleMessage(e.data);
    };
    ws.addEventListener("message", messageListener);

    const closeListener = (e: CloseEvent) => {
      setStatus(ConnectionStatus.Close);
      setCloseReason(e.reason);
      console.log("close event", e);
    };
    ws.addEventListener("close", closeListener);

    setConnection((prev) => ({ ...prev, ws }));

    return () => {
      ws.removeEventListener("message", messageListener);
      ws.removeEventListener("close", closeListener);
      ws.close();
      setConnection((prev) => ({ ...prev, ws: undefined }));
    };
  }, [
    connectionOrigin,
    confirmInstallPlugin,
    confirmDeletePlugin,
    setConnection,
  ]);

  if (status === ConnectionStatus.Pending) {
    return <ConnectionPending />;
  }

  if (status === ConnectionStatus.Close) {
    return <ConnectionClose reason={closeReason} />;
  }

  return <>{children}</>;
}
