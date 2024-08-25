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
      <p className="my-2">
        <b className="mr-2">Daemon 服务</b>
        <span>{isolate?.daemon_addr}</span>
      </p>
      <div>
        {plugins?.map((item) => (
          <div key={item.config.name} className="mb-2">
            <PluginCard plugin={item} />
          </div>
        ))}
      </div>
    </div>
  );
}
