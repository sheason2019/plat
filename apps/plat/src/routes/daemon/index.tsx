import { useParams } from "react-router-dom";
import useDaemonScopes from "../../hooks/use-daemons";

export default function DaemonPage() {
  const { data: scopes } = useDaemonScopes();
  const { daemonPublicKey } = useParams();

  const scope = scopes.find(
    (item) => item.daemon.public_key === daemonPublicKey
  );

  if (!scope) {
    return (
      <div className="w-full h-full flex justify-center items-center">
        无法获取 Daemon Scope 信息
      </div>
    );
  }

  return <iframe src={scope.daemon.address} className="w-full h-full" />;
}
