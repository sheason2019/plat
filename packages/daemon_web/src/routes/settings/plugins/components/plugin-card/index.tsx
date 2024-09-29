import {
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
} from "@nextui-org/react";
import { IPlugin } from "../../../../../components/connection-provider/typings";

interface Props {
  plugin: IPlugin;
}

export default function PluginCard({ plugin }: Props) {
  return (
    <Card>
      <CardHeader>
        <p>{plugin.name}</p>
        <p>{plugin.version}</p>
      </CardHeader>
      <CardBody>
        <p>Plugin Address: {plugin.address}</p>
        <p>Entry Count: {plugin.entries.length}</p>
      </CardBody>
      <CardFooter>
        <Button>卸载</Button>
      </CardFooter>
    </Card>
  );
}
