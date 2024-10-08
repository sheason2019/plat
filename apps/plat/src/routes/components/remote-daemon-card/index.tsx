import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Divider,
} from "@nextui-org/react";
import { RemoteDaemon } from "../../../models/core";
import DeleteDaemonButton from "../local-daemon-card/delete-daemon-button";
import useOpenDaemon from "../../hooks/use-open-daemon";

interface Props {
  remoteDaemon: RemoteDaemon;
}

export default function RemoteDaemonCard({ remoteDaemon }: Props) {
  const { openDaemon } = useOpenDaemon();

  return (
    <Card>
      <CardHeader>
        <p className="font-mono font-bold tracking-wide w-full overflow-hidden text-ellipsis whitespace-nowrap">
          {remoteDaemon.address}
        </p>
      </CardHeader>
      <Divider />
      <CardBody>
        <div className="text-default-500 text-sm">
          <p>Type: Remote</p>
        </div>
      </CardBody>
      <CardFooter>
        <DeleteDaemonButton address={remoteDaemon.address} />
        <div className="flex-1" />
        <Button color="primary" onClick={() => openDaemon(remoteDaemon)}>
          进入
        </Button>
      </CardFooter>
    </Card>
  );
}
