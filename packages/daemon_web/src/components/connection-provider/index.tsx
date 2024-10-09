import { PropsWithChildren, useEffect, useState } from "react";
import ConnectionPending from "./pending";
import { ConnectionStatus, IDaemon } from "./typings";
import ConnectionClose from "./close";
import { useSetRecoilState } from "recoil";
import { connectionState } from "./context";
import useConfirmModal from "../confirm-modal/hooks/use-confirm-modal";

export default function ConnectionProvider({ children }: PropsWithChildren) {
  const [status, setStatus] = useState(ConnectionStatus.Pending);
  const [closeReason, setCloseReason] = useState("");
  const { confirmInstallPlugin } = useConfirmModal();

  const setConnection = useSetRecoilState(connectionState);

  useEffect(() => {
    setStatus(ConnectionStatus.Pending);

    const ws = new WebSocket(
      `${location.origin.replace("http", "ws")}/api/connect`
    );

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
  }, [confirmInstallPlugin, setConnection]);

  if (status === ConnectionStatus.Pending) {
    return <ConnectionPending />;
  }

  if (status === ConnectionStatus.Close) {
    return <ConnectionClose reason={closeReason} />;
  }

  return <>{children}</>;
}
