import { Button } from "@nextui-org/react";
import { Entry, Plugin } from "../../models/core";
import useStage from "../stage/hooks/use-stage";

interface Props {
  plugin: Plugin;
  entry: Entry;
}

export default function EntryButton({ entry, plugin }: Props) {
  const [, setStage] = useStage();

  return (
    <Button
      isIconOnly
      className="w-16 h-16"
      variant="light"
      onClick={() => setStage({ plugin, entry })}
    >
      {entry.name}
    </Button>
  );
}
