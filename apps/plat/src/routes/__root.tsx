import { NextUIProvider } from "@nextui-org/react";
import { createRootRoute, Outlet, useNavigate } from "@tanstack/react-router";
import { RecoilRoot } from "recoil";
import ChannelProvider from "../components/channel-provider";

export const Route = createRootRoute({
  component: Root,
});

function Root() {
  const navigate = useNavigate();

  const handleNavigate = (path: string) => {
    navigate({ to: path });
  };

  return (
    <RecoilRoot>
      <NextUIProvider id="next-provider" navigate={handleNavigate}>
        <ChannelProvider>
          <Outlet />
        </ChannelProvider>
      </NextUIProvider>
    </RecoilRoot>
  );
}
