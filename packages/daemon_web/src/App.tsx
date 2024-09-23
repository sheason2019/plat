import { NextUIProvider } from "@nextui-org/react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import DaemonLayout from "./routes/layout";
import IndexPage from "./routes";
import DaemonContextProvider from "./components/daemon-context/provider";
import { RecoilRoot } from "recoil";

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
    <RecoilRoot>
      <DaemonContextProvider>
        <NextUIProvider>
          <RouterProvider router={router} />
        </NextUIProvider>
      </DaemonContextProvider>
    </RecoilRoot>
  );
}

export default App;
