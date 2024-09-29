import { Button } from "@nextui-org/react";
import { IPlugin, IPluginEntry } from "../connection-provider/typings";
import { Link } from "react-router-dom";

interface Props {
  plugin: IPlugin;
  entry: IPluginEntry;
  onClose(): void;
}

export default function PluginEntry({ plugin, entry, onClose }: Props) {
  return (
    <Button
      startContent={<img src={plugin.address + entry.icon} />}
      as={Link}
      to={`/plugin/${encodeURIComponent(plugin.name)}/${encodeURIComponent(
        entry.label
      )}`}
      onClick={onClose}
    >
      {entry.label}
    </Button>
  );
}
