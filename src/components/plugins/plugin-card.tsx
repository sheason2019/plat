import { Card, CardBody, CardFooter } from "@nextui-org/react";
import { RegistedPlugin } from "../../models/core";
import DeletePluginButton from "./delete-plugin-button";

interface Props {
  plugin: RegistedPlugin;
}

export default function PluginCard({ plugin }: Props) {
  return (
    <Card>
      <CardBody>
        <p className="font-bold mb-2">{plugin.config.name}</p>
        <p>当前运行端口：{plugin.addr}</p>
        <p>插件定义入口：{plugin.config.entries.length}</p>
      </CardBody>
      <CardFooter className="pt-0 justify-end">
        <DeletePluginButton plugin={plugin} />
      </CardFooter>
    </Card>
  );
}
