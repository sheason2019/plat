import { createBrowserRouter, RouterProvider } from "react-router-dom";
import DaemonLayout from "./routes/layout";
import IndexPage from "./routes";
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
      <RouterProvider router={router} />
    </RecoilRoot>
  );
}

export default App;
