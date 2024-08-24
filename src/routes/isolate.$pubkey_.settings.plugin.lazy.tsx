import { createLazyFileRoute } from "@tanstack/react-router";
import useIsolate from "../hooks/core/use-isolate";
import PluginCard from "../components/plugins/plugin-card";
import InstallPluginButton from "../components/plugins/install-plugin-button";

export const Route = createLazyFileRoute("/isolate/$pubkey/settings/plugin")({
  component: PluginManage,
});

function PluginManage() {
  const isolate = useIsolate();
  const plugins = isolate?.plugins && Object.values(isolate?.plugins);

  return (
    <div className="container px-2 mx-auto">
      <div className="flex items-center my-3">
        <h1 className="text-xl flex-1">已安装的插件</h1>
        <InstallPluginButton />
      </div>
      <div>
        {plugins?.map((item) => (
          <PluginCard plugin={item} key={item.config.name} />
        ))}
      </div>
    </div>
  );
}
