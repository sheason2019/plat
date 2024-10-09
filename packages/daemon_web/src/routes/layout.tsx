import { Outlet, useNavigate } from "react-router-dom";
import DaemonControl from "../components/daemon-control";
import ConnectionProvider from "../components/connection-provider";
import { NextUIProvider } from "@nextui-org/react";
import ConfirmModal from "../components/confirm-modal";

export default function DaemonLayout() {
  const navigate = useNavigate();

  return (
    <NextUIProvider id="next-ui-provider" navigate={navigate}>
      <ConfirmModal />
      <DaemonControl />
      <ConnectionProvider>
        <Outlet />
      </ConnectionProvider>
    </NextUIProvider>
  );
}
