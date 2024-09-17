import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { NextUIProvider } from "@nextui-org/react";
import { RecoilRoot } from "recoil";
import "@fontsource-variable/noto-sans-sc";

import "./main.css";
import IndexPage from "./routes";
import PlatProvider from "./components/plat-provider";
import DaemonPage from "./routes/daemon";

const router = createBrowserRouter([
  {
    path: "/",
    element: <IndexPage />,
  },
  {
    path: "/:daemonPublicKey",
    element: <DaemonPage />,
  },
]);

// Render the app
const rootElement = document.getElementById("root")!;
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <RecoilRoot>
        <NextUIProvider id="next-provider">
          <PlatProvider>
            <RouterProvider router={router} />
          </PlatProvider>
        </NextUIProvider>
      </RecoilRoot>
    </StrictMode>
  );
}
