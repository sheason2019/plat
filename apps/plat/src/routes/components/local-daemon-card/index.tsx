import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Divider,
  Link,
} from "@nextui-org/react";
import { Daemon } from "../../../models/core";
import DeleteDaemonButton from "./delete-daemon-button";

interface Props {
  daemon: Daemon;
}

export default function LocalDaemonCard({ daemon }: Props) {
  return (
    <Card>
      <CardHeader>
        <p className="font-mono font-bold tracking-wide w-full overflow-hidden text-ellipsis whitespace-nowrap">
          {daemon.public_key}
        </p>
      </CardHeader>
      <Divider />
      <CardBody>
        <div className="text-default-500 text-sm">
          <p>Type: Local</p>
          <p>Address: {daemon.address}</p>
        </div>
      </CardBody>
      <CardFooter>
        <DeleteDaemonButton publicKey={daemon.public_key} />
        <div className="flex-1" />
        <Button
          color="primary"
          as={Link}
          href={`/daemons/local/${daemon.public_key}`}
        >
          进入
        </Button>
      </CardFooter>
    </Card>
  );
}
