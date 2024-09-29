import { createBrowserRouter, RouterProvider } from "react-router-dom";
import DaemonLayout from "./routes/layout";
import IndexPage from "./routes";
import DaemonContextProvider from "./components/daemon-context/provider";
import { RecoilRoot } from "recoil";
import PluginPage from "./routes/plugin";
import SettingsPage from "./routes/settings";

const router = createBrowserRouter([
  {
    path: "/",
    element: <DaemonLayout />,
    children: [
      {
        path: "",
        element: <IndexPage />,
      },
      {
        path: "settings",
        element: <SettingsPage />,
      },
      {
        path: "plugin/:pluginName/:entryLabel",
        element: <PluginPage />,
      },
    ],
  },
]);

function App() {
  return (
    <RecoilRoot>
      <DaemonContextProvider>
        <RouterProvider router={router} />
      </DaemonContextProvider>
    </RecoilRoot>
  );
}

export default App;
