import { NextUIProvider } from "@nextui-org/react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import DaemonLayout from "./routes/layout";
import IndexPage from "./routes";

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
    <NextUIProvider>
      <RouterProvider router={router} />
    </NextUIProvider>
  );
}

export default App;
