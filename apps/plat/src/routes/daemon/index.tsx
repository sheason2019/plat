import { useEffect, useRef } from "react";
import { useNavigate, useParams } from "react-router-dom";
import useDaemonScopes from "../../hooks/use-daemons";

export default function DaemonPage() {
  const { daemonKey } = useParams();
  const { findByDaemonKey } = useDaemonScopes();
  const scope = findByDaemonKey(daemonKey!)!;

  const navigate = useNavigate();
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    const postMessageHandler = (e: MessageEvent) => {
      if (e.data?.type === "exit") {
        return navigate("/");
      }
    };
    window.addEventListener("message", postMessageHandler);
    return () => window.removeEventListener("message", postMessageHandler);
  }, []);

  useEffect(() => {
    const el = iframeRef.current;
    if (!el) return;

    const onload = () => {
      console.log("password", scope.daemon.password);
      el.contentWindow?.postMessage(
        JSON.stringify({
          type: "context",
          payload: {
            fromOrigin: location.origin,
            password: scope.daemon.password,
            publicKey: scope.daemon.public_key,
          },
        }),
        new URL(scope.daemon.address).origin
      );
    };
    el.addEventListener("load", onload);
    return () => el.removeEventListener("load", onload);
  }, [iframeRef]);

  return (
    <iframe
      ref={iframeRef}
      src={scope.daemon.address}
      className="w-full h-full"
    />
  );
}
