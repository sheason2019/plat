import { useParams } from "react-router-dom";
import useDaemons from "../../../hooks/use-daemons";
import DaemonFrame from "../common/daemon-frame";

export default function LocalDaemonPage() {
  const { publicKey } = useParams();
  const { daemons } = useDaemons();
  const daemon = daemons.local_daemons.find(
    (item) => item.public_key === publicKey
  )!;

  return <DaemonFrame address={daemon.address} password={daemon.password} />;
}
