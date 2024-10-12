import { useEffect, useRef } from "react";
import { useNavigate } from "react-router-dom";
import WujieReact from "wujie-react";

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

  return (
    <WujieReact
      width="100%"
      height="100%"
      name={`daemon/${address}`}
      url={address}
      sync={true}
    />
  );
}
