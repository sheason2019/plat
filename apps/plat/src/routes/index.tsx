import DaemonCard from "./components/daemon-card";
import useDaemonScopes from "../hooks/use-daemons";
import Header from "./components/header";
import CreateDaemon from "./components/create-daemon";

export default function IndexPage() {
  const { scopes, getDaemonKey } = useDaemonScopes();

  return (
    <div className="container mx-auto px-2">
      <Header title="账号管理" />
      <div className="grid grid-cols-1 md:grid-cols-2 mt-3 gap-3">
        {scopes?.map((scope) => (
          <DaemonCard key={getDaemonKey(scope)} scope={scope} />
        ))}
      </div>
      <CreateDaemon />
    </div>
  );
}
