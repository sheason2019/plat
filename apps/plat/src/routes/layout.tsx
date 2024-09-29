import { NextUIProvider } from "@nextui-org/react";
import { Outlet, useNavigate } from "react-router-dom";
import { RecoilRoot } from "recoil";
import PlatProvider from "../components/plat-provider";

export default function Layout() {
  const navigate = useNavigate();

  return (
    <RecoilRoot>
      <NextUIProvider id="next-provider" navigate={navigate}>
        <PlatProvider>
          <Outlet />
        </PlatProvider>
      </NextUIProvider>
    </RecoilRoot>
  );
}
