import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Divider,
} from "@nextui-org/react";
import { Daemon } from "../../../models/core";
import DeleteDaemonButton from "./delete-daemon-button";
import useOpenDaemon from "../../hooks/use-open-daemon";

interface Props {
  daemon: Daemon;
}

export default function LocalDaemonCard({ daemon }: Props) {
  const { openDaemon } = useOpenDaemon();

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
        </div>
      </CardBody>
      <CardFooter>
        <DeleteDaemonButton publicKey={daemon.public_key} />
        <div className="flex-1" />
        <Button color="primary" onClick={() => openDaemon(daemon)}>
          进入
        </Button>
      </CardFooter>
    </Card>
  );
}
