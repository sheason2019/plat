import { useEffect, useRef } from "react";
import { useNavigate } from "react-router-dom";

interface Props {
  address: string;
}

export default function DaemonFrame({ address }: Props) {
  const navigate = useNavigate();
  const iframeRef = useRef<HTMLIFrameElement>(null);
  useEffect(() => {
    const el = iframeRef.current;
    if (!el) return;

    const handler = (e: MessageEvent) => {
      try {
        switch (e.data.type) {
          case "exit":
            navigate("/");
            break;
          default:
            break;
        }
      } catch (err) {
        console.error("handle message error:", err);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [iframeRef]);

  return <iframe ref={iframeRef} src={address} className="w-full h-full" />;
}
