import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Divider,
} from "@nextui-org/react";
import { DaemonScope } from "../../../models/core";
import DeleteDaemonButton from "./delete-daemon-button";
import { Link } from "react-router-dom";

interface Props {
  scope: DaemonScope;
}

export default function LocalDaemonCard({ scope }: Props) {
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
          <p>{scope.daemon.address}</p>
        </div>
      </CardBody>
      <CardFooter>
        <DeleteDaemonButton scope={scope} />
        <div className="flex-1" />
        <Button
          color="primary"
          as={Link}
          to={`/daemon/${scope.daemon.public_key}`}
        >
          进入
        </Button>
      </CardFooter>
    </Card>
  );
}
