import { NextUIProvider } from "@nextui-org/react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import DaemonLayout from "./routes/layout";
import IndexPage from "./routes";
import ConnectionProvider from "./components/connection-provider";

const router = createBrowserRouter([
  {
    path: "/",
    element: <DaemonLayout />,
    children: [
      {
        path: "",
        element: <IndexPage />,
      },
    ],
  },
]);

function App() {
  return (
    <ConnectionProvider>
      <NextUIProvider>
        <RouterProvider router={router} />
      </NextUIProvider>
    </ConnectionProvider>
  );
}

export default App;
