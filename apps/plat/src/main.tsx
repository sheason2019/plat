import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "@fontsource-variable/noto-sans-sc";

import "./main.css";
import IndexPage from "./routes";
import Layout from "./routes/layout";
import TemplatesPage from "./routes/templates";
import LocalDaemonPage from "./routes/daemon/local";
import RemoteDaemonPage from "./routes/daemon/remote";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        path: "",
        element: <IndexPage />,
      },
      {
        path: "templates",
        element: <TemplatesPage />,
      },
      {
        path: "daemons/local/:publicKey",
        element: <LocalDaemonPage />,
      },
      {
        path: "daemons/remote/:address",
        element: <RemoteDaemonPage />,
      },
    ],
  },
]);

// Render the app
const rootElement = document.getElementById("root")!;
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <RouterProvider router={router} />
    </StrictMode>
  );
}
