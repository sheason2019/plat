import { useEffect, useMemo, useRef } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";

export default function DaemonPage() {
  const [search] = useSearchParams();
  const navigate = useNavigate();
  const iframeRef = useRef<HTMLIFrameElement>(null);

  const address = useMemo(() => {
    const address = search.get("address");
    if (!address) return null;
    const password = search.get("password");

    const addressUrl = new URL(decodeURIComponent(address));
    addressUrl.searchParams.set("fromOrigin", location.origin);
    addressUrl.searchParams.set("password", password ?? "");
    return addressUrl;
  }, [search.get("address")]);

  useEffect(() => {
    const postMessageHandler = (e: MessageEvent) => {
      if (e.data?.type === "exit") {
        return navigate("/");
      }
    };
    window.addEventListener("message", postMessageHandler);
    return () => window.removeEventListener("message", postMessageHandler);
  }, []);

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
