import { useEffect, useRef } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";

export default function DaemonPage() {
  const [search] = useSearchParams();
  const navigate = useNavigate();
  const iframeRef = useRef<HTMLIFrameElement>(null);

  const address = new URL(decodeURIComponent(search.get("address")!));

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
      el.contentWindow?.postMessage(
        JSON.stringify({
          type: "context",
          payload: {
            fromOrigin: location.origin,
            password: search.get("password"),
          },
        }),
        address.origin
      );
    };
    el.addEventListener("load", onload);
    return () => el.removeEventListener("load", onload);
  }, [iframeRef]);

  if (!address) {
    return (
      <div className="w-full h-full flex justify-center items-center">
        无效 Daemon 地址
      </div>
    );
  }

  return (
    <iframe
      ref={iframeRef}
      src={address.toString()}
      className="w-full h-full"
    />
  );
}
