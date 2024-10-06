import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "@fontsource-variable/noto-sans-sc";

import "./main.css";
import IndexPage from "./routes";
import LocalDaemonPage from "./routes/local-daemon";
import Layout from "./routes/layout";
import TemplatesPage from "./routes/templates";

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
        path: "/daemons/local/:daemonKey",
        element: <LocalDaemonPage />,
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
