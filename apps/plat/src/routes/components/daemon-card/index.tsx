import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Divider,
} from "@nextui-org/react";
import { DaemonScope, DaemonVariant } from "../../../models/core";
import DeleteDaemonButton from "./delete-daemon-button";
import EditDaemonButton from "./edit-daemon-button";
import useDaemonScopes from "../../../hooks/use-daemons";
import { Link } from "react-router-dom";

interface Props {
  scope: DaemonScope;
}

export default function DaemonCard({ scope }: Props) {
  const { getDaemonKey } = useDaemonScopes();
  const daemonKey = decodeURIComponent(getDaemonKey(scope));

  return (
    <Card>
      <CardHeader>
        <p className="font-mono font-bold tracking-wide w-full overflow-hidden text-ellipsis whitespace-nowrap">
          {daemonKey}
        </p>
      </CardHeader>
      <Divider />
      <CardBody>
        <div className="text-default-500 text-sm">
          <p>账号类型：{scope.daemon.variant}</p>
          <p>插件数量：{scope.plugins.length}</p>
          {scope.daemon.variant === DaemonVariant.Local && (
            <p>账号地址：{scope.daemon.address}</p>
          )}
        </div>
      </CardBody>
      <CardFooter>
        <DeleteDaemonButton daemonKey={daemonKey!} />
        {scope.daemon.variant === DaemonVariant.Remote && (
          <EditDaemonButton daemonKey={daemonKey!} />
        )}
        <div className="flex-1" />
        <Button
          color="primary"
          as={Link}
          to={`/daemon/${encodeURIComponent(daemonKey)}`}
        >
          进入
        </Button>
      </CardFooter>
    </Card>
  );
}
