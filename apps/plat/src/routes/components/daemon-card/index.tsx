import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Divider,
} from "@nextui-org/react";
import { DaemonScope } from "../../../models/core";

interface Props {
  scope: DaemonScope;
}

export default function IsolateCard({ scope }: Props) {
  return (
    <Card>
      <CardHeader>
        <p className="font-mono font-bold tracking-wide w-full overflow-hidden text-ellipsis">
          {scope.daemon.public_key}
        </p>
      </CardHeader>
      <Divider />
      <CardBody>
        <div className="text-default-500 text-sm">
          <p>账号类型：{scope.daemon.variant}</p>
          <p>插件数量：{scope.plugins.length}</p>
          <p>账号地址：{scope.daemon.address}</p>
        </div>
      </CardBody>
      <CardFooter>
        <Button isIconOnly></Button>
        <div className="flex-1" />
        <Button>进入</Button>
      </CardFooter>
    </Card>
  );
}
