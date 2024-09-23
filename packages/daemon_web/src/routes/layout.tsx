import { Outlet } from "react-router-dom";
import DaemonControl from "../components/daemon-control";
import ConnectionProvider from "../components/connection-provider";

export default function DaemonLayout() {
  return (
    <>
      <DaemonControl />
      <ConnectionProvider>
        <Outlet />
      </ConnectionProvider>
    </>
  );
}
