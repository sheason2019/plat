import { useEffect, useRef } from "react";
import { useNavigate } from "react-router-dom";

interface Props {
  address: string;
  password: string;
}

export default function DaemonFrame({ address, password }: Props) {
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
            password: password,
          },
        }),
        new URL(address).origin
      );
      i++;
    }, 100);

    return () => clearInterval(intervalRef.current);
  }, [address, password]);

  return <iframe ref={iframeRef} src={address} className="w-full h-full" />;
}
