import { Outlet } from "react-router-dom";
import DaemonControl from "../components/daemon-control";
import ConnectionProvider from "../components/connection-provider";
import { useDaemonContext } from "../components/daemon-context/context";

export default function DaemonLayout() {
  const context = useDaemonContext();

  return (
    <>
      <DaemonControl />
      {context?.password ? (
        <ConnectionProvider>
          <Outlet />
        </ConnectionProvider>
      ) : (
        <p>TODO: 输入密码</p>
      )}
    </>
  );
}
