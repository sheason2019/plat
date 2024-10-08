import { useParams } from "react-router-dom";
import DaemonFrame from "../common/daemon-frame";
import useDaemons from "../../../hooks/use-daemons";

export default function RemoteDaemonPage() {
  const { address } = useParams();
  const { daemons } = useDaemons();
  const addressString = decodeURIComponent(address ?? "");
  const daemon = daemons.remote_daemons.find(
    (item) => item.address === addressString
  )!;

  return <DaemonFrame address={daemon.address} />;
}
