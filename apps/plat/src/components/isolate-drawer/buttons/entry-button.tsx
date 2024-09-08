import { PluginEntry, RegistedPlugin } from "../../../models/core";
import useStage from "../../stage/hooks/use-stage";
import useIsolateDrawer from "../hooks/use-isolate-drawer";
import DrawerButton from "./drawer-button";

interface Props {
  plugin: RegistedPlugin;
  entry: PluginEntry;
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
      <div className="flex flex-col gap-1 items-center justify-center">
        {entry.icon.length > 0 ? (
          <img className="w-6 h-6" src={plugin.addr + entry.icon} />
        ) : (
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            viewBox="0 0 16 16"
            className="w-6 h-6"
          >
            <path d="M8.186 1.113a.5.5 0 0 0-.372 0L1.846 3.5 8 5.961 14.154 3.5 8.186 1.113zM15 4.239l-6.5 2.6v7.922l6.5-2.6V4.24zM7.5 14.762V6.838L1 4.239v7.923l6.5 2.6zM7.443.184a1.5 1.5 0 0 1 1.114 0l7.129 2.852A.5.5 0 0 1 16 3.5v8.662a1 1 0 0 1-.629.928l-7.185 2.874a.5.5 0 0 1-.372 0L.63 13.09a1 1 0 0 1-.63-.928V3.5a.5.5 0 0 1 .314-.464L7.443.184z" />
          </svg>
        )}
        <p className="text-xs">{entry.label}</p>
      </div>
    </DrawerButton>
  );
}
