import { useSearchParams } from "react-router-dom";

export default function DaemonPage() {
  const [search, setSearch] = useSearchParams();
  const address = search.get("address");

  if (!address) {
    return (
      <div className="w-full h-full flex justify-center items-center">
        无效 Daemon 地址
      </div>
    );
  }

  return <iframe src={address} className="w-full h-full" />;
}
