import { useMemo } from "react";
import { useParams } from "react-router-dom";
import { useRecoilValue } from "recoil";
import { connectionState } from "../../components/connection-provider/context";

export default function PluginPage() {
  const connection = useRecoilValue(connectionState);
  const { pluginName, entryLabel } = useParams();

  const address = useMemo(() => {
    const plugin = connection?.daemon.plugins.find(
      (item) => item.name === pluginName
    );
    const entry = plugin?.entries.find((item) => item.label === entryLabel);

    return `${plugin?.address}${entry?.href}`;
  }, [connection?.daemon.plugins, entryLabel, pluginName]);

  return <iframe className="h-full h-full-parent w-full" src={address} />;
}
