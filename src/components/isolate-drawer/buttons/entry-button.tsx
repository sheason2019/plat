import { Entry, RegistedPlugin } from "../../../models/core";
import useStage from "../../stage/hooks/use-stage";
import useIsolateDrawer from "../hooks/use-isolate-drawer";
import DrawerButton from "./drawer-button";

interface Props {
  plugin: RegistedPlugin;
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
      {entry.label}
    </DrawerButton>
  );
}
