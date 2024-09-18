import { Outlet } from "react-router-dom";
import DaemonControl from "../components/daemon-control";

export default function DaemonLayout() {
  return (
    <>
      <DaemonControl />
      <Outlet />
    </>
  );
}
