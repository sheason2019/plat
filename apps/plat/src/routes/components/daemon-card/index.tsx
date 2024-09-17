import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Divider,
  Link,
} from "@nextui-org/react";
import { DaemonScope } from "../../../models/core";
import DeleteDaemonButton from "./delete-daemon-button";

interface Props {
  scope: DaemonScope;
}

export default function IsolateCard({ scope }: Props) {
  return (
    <Card>
      <CardHeader>
        <p className="font-mono font-bold tracking-wide w-full overflow-hidden text-ellipsis whitespace-nowrap">
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
        <DeleteDaemonButton publicKey={scope.daemon.public_key} />
        <div className="flex-1" />
        <Button color="primary" as={Link} href={`/${scope.daemon.public_key}`}>
          进入
        </Button>
      </CardFooter>
    </Card>
  );
}
