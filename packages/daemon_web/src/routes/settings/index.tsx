import { useRecoilValue } from "recoil";
import { connectionState } from "../../components/connection-provider/context";
import PluginCard from "./plugins/components/plugin-card";
import InstallPluginButton from "./plugins/components/install-plugin-button";

export default function SettingsPage() {
  const connection = useRecoilValue(connectionState);

  return (
    <div className="container px-3 mx-auto">
      <h1 className="h-10 flex items-center text-lg whitespace-nowrap select-none">
        Daemon 设置 / 插件管理
      </h1>
      <InstallPluginButton />
      <div className="grid grid-cols-1 gap-4 mt-4">
        {connection?.daemon.plugins.map((plugin) => (
          <PluginCard key={plugin.name} plugin={plugin} />
        ))}
      </div>
    </div>
  );
}
