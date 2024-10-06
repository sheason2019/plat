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
import { Link } from "react-router-dom";

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
          <p>{daemon.address}</p>
        </div>
      </CardBody>
      <CardFooter>
        <DeleteDaemonButton daemon={daemon} />
        <div className="flex-1" />
        <Button
          color="primary"
          as={Link}
          to={`/daemons/local/${daemon.public_key}`}
        >
          进入
        </Button>
      </CardFooter>
    </Card>
  );
}
