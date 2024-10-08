import { PropsWithChildren, useEffect, useRef, useState } from "react";
import ConnectionPending from "./pending";
import { ConnectionStatus, IDaemon } from "./typings";
import ConnectionClose from "./close";
import { useSetRecoilState } from "recoil";
import { connectionState } from "./context";

export default function ConnectionProvider({ children }: PropsWithChildren) {
  const wsRef = useRef<WebSocket | null>(null);

  const [status, setStatus] = useState(ConnectionStatus.Pending);
  const [closeReason, setCloseReason] = useState("");

  const setConnection = useSetRecoilState(connectionState);

  useEffect(() => {
    setStatus(ConnectionStatus.Pending);

    const ws = new WebSocket(
      `${location.origin.replace("http", "ws")}/api/connect`
    );

    const handleMessage = async (message: string) => {
      const data: {
        type: string;
        payload: object;
      } = JSON.parse(message);
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

    wsRef.current = ws;

    return () => {
      ws.removeEventListener("message", messageListener);
      ws.removeEventListener("close", closeListener);
      ws.close();
      wsRef.current = null;
    };
  }, [setConnection]);

  if (status === ConnectionStatus.Pending) {
    return <ConnectionPending />;
  }

  if (status === ConnectionStatus.Close) {
    return <ConnectionClose reason={closeReason} />;
  }

  return <>{children}</>;
}
