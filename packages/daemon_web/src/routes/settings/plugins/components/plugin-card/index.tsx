import { Button, Card, CardBody, CardFooter } from "@nextui-org/react";
import { IPlugin } from "../../../../../components/connection-provider/typings";
import axios from "axios";

interface Props {
  plugin: IPlugin;
}

export default function PluginCard({ plugin }: Props) {
  const handleDelete = () =>
    axios.delete("/api/plugin", { params: { name: plugin.name } });

  return (
    <Card>
      <CardBody>
        <pre>{JSON.stringify(plugin, null, "  ")}</pre>
      </CardBody>
      <CardFooter>
        <Button color="danger" onClick={handleDelete}>
          删除
        </Button>
      </CardFooter>
    </Card>
  );
}
