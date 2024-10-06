import { useEffect, useRef } from "react";
import { useNavigate, useParams } from "react-router-dom";
import useDaemonScopes from "../../hooks/use-daemons";

export default function LocalDaemonPage() {
  const { daemonKey } = useParams();
  const { scopes } = useDaemonScopes();
  const daemon = scopes.local_daemons.find(
    (item) => item.public_key === daemonKey
  )!;

  const navigate = useNavigate();
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const intervalRef = useRef<number>();

  useEffect(() => {
    const postMessageHandler = (e: MessageEvent) => {
      if (e.data?.type === "exit") {
        return navigate("/");
      }
      if (e.data?.type === "context-received") {
      }
    };
    window.addEventListener("message", postMessageHandler);
    return () => window.removeEventListener("message", postMessageHandler);
  }, []);

  useEffect(() => {
    let i = 0;
    intervalRef.current = setInterval(() => {
      if (i > 5) {
        return clearInterval(intervalRef.current);
      }

      iframeRef.current?.contentWindow?.postMessage(
        JSON.stringify({
          type: "context",
          payload: {
            fromOrigin: location.origin,
            password: daemon.password,
            publicKey: daemon.public_key,
          },
        }),
        new URL(daemon.address).origin
      );
      i++;
    }, 100);

    return () => clearInterval(intervalRef.current);
  }, [daemon]);

  return (
    <iframe ref={iframeRef} src={daemon.address} className="w-full h-full" />
  );
}
