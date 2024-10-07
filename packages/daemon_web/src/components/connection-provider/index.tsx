import { PropsWithChildren, useEffect, useRef, useState } from "react";
import { sha3_256 } from "@noble/hashes/sha3";
import { base64url } from "@scure/base";
import ConnectionPending from "./pending";
import { ConnectionStatus, IDaemon } from "./typings";
import ConnectionClose from "./close";
import { useDaemonContext } from "../daemon-context/context";
import { useSetRecoilState } from "recoil";
import { connectionState } from "./context";

export default function ConnectionProvider({ children }: PropsWithChildren) {
  const wsRef = useRef<WebSocket | null>(null);

  const context = useDaemonContext();

  const [status, setStatus] = useState(ConnectionStatus.Pending);
  const [closeReason, setCloseReason] = useState("");

  const setConnection = useSetRecoilState(connectionState);

  useEffect(() => {
    setStatus(ConnectionStatus.Pending);
    let sequence = 0;

    const ws = new WebSocket(
      `${location.origin.replace("http", "ws")}/api/connect`
    );

    const handleReceiveSalt = async (salt: string) => {
      const passwordBuf = new TextEncoder().encode(context?.password);
      const saltBuf = new TextEncoder().encode(salt);
      const passwordHash = sha3_256
        .create()
        .update(passwordBuf)
        .update(saltBuf)
        .digest();

      ws.send(base64url.encode(passwordHash));
    };

    const handleReceiveResult = async (result: string) => {
      if (result === "OK") {
        return setStatus(ConnectionStatus.Open);
      }
    };

    const handleMessage = async (message: string) => {
      const data: {
        type: string;
        payload: object;
      } = JSON.parse(message);
      switch (data.type) {
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
      switch (sequence) {
        case 0:
          await handleReceiveSalt(e.data);
          break;
        case 1:
          await handleReceiveResult(e.data);
          break;
        default:
          await handleMessage(e.data);
          break;
      }

      sequence++;
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
  }, [context?.password, setConnection]);

  if (status === ConnectionStatus.Pending) {
    return <ConnectionPending />;
  }

  if (status === ConnectionStatus.Close) {
    return <ConnectionClose reason={closeReason} />;
  }

  return <>{children}</>;
}
