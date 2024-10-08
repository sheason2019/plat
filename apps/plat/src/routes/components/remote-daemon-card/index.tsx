import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Divider,
  Link,
} from "@nextui-org/react";
import { RemoteDaemon } from "../../../models/core";
import DeleteDaemonButton from "../local-daemon-card/delete-daemon-button";

interface Props {
  remoteDaemon: RemoteDaemon;
}

export default function RemoteDaemonCard({ remoteDaemon }: Props) {
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
        <Button
          color="primary"
          as={Link}
          href={`/daemons/remote/${encodeURIComponent(remoteDaemon.address)}`}
        >
          进入
        </Button>
      </CardFooter>
    </Card>
  );
}
