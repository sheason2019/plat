import { Button } from "@nextui-org/react";
import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/isolate/$pubkey/settings/plugin")({
  component: PluginManage,
});

function PluginManage() {
  return (
    <div className="container px-2 mx-auto">
      <div className="flex items-center my-3">
        <h1 className="text-xl flex-1">已安装的插件</h1>
        <Button color="primary">添加插件</Button>
      </div>
      <div style={{ height: "1600px" }} />
    </div>
  );
}
