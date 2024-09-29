import DaemonCard from "./components/daemon-card";
import useDaemonScopes from "../hooks/use-daemons";
import Header from "./components/header";
import CreateDaemon from "./components/create-daemon";
import Navigation from "../components/navigation";

export default function IndexPage() {
  const { scopes, getDaemonKey } = useDaemonScopes();

  return (
    <div className="container mx-auto px-2">
      <Header
        title={
          <div className="flex items-center gap-2">
            <Navigation />
            <p>账号管理</p>
          </div>
        }
      />
      <div className="grid grid-cols-1 md:grid-cols-2 mt-3 gap-3">
        {scopes?.map((scope) => (
          <DaemonCard key={getDaemonKey(scope)} scope={scope} />
        ))}
      </div>
      <CreateDaemon />
    </div>
  );
}
