import { Entry, Plugin } from "../../../models/core";
import useStage from "../../stage/hooks/use-stage";
import useIsolateDrawer from "../hooks/use-isolate-drawer";
import DrawerButton from "./drawer-button";

interface Props {
  plugin: Plugin;
  entry: Entry;
}

export default function EntryButton({ entry, plugin }: Props) {
  const [, setStage] = useStage();
  const { onClose } = useIsolateDrawer();

  return (
    <DrawerButton
      onClick={() => {
        setStage({ plugin, entry });
        onClose();
      }}
    >
      {entry.name}
    </DrawerButton>
  );
}
