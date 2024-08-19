import { Button, Card, CardBody, CardFooter } from "@nextui-org/react";
import { PlatX } from "../../models/core";

interface Props {
  plugin: PlatX;
}

export default function PluginCard({ plugin }: Props) {
  return (
    <Card>
      <CardBody>
        <p className="font-bold mb-2">{plugin.config.name}</p>
        <p>当前运行端口：{plugin.port}</p>
        <p>插件定义入口：{plugin.config.entries.length}</p>
      </CardBody>
      <CardFooter className="pt-0 justify-end">
        <Button color="danger">卸载</Button>
      </CardFooter>
    </Card>
  );
}
