import { Button, Link } from "@nextui-org/react";
import { IPlugin, IPluginEntry } from "../connection-provider/typings";

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
      href={`/plugin/${encodeURIComponent(plugin.name)}/${encodeURIComponent(
        entry.label
      )}`}
      onClick={onClose}
    >
      {entry.label}
    </Button>
  );
}
