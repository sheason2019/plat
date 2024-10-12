import { useMemo } from "react";
import { useParams } from "react-router-dom";
import { useRecoilValue } from "recoil";
import { connectionState } from "../../components/connection-provider/context";
import WujieReact from "wujie-react";

export default function PluginPage() {
  const connection = useRecoilValue(connectionState);
  const { pluginName, entryLabel } = useParams();

  const address = useMemo(() => {
    const plugin = connection.daemon?.plugins.find(
      (item) => item.name === pluginName
    );
    const entry = plugin?.entries.find((item) => item.label === entryLabel);

    return `${plugin?.address}${entry?.href}`;
  }, [connection.daemon?.plugins, entryLabel, pluginName]);

  return (
    <WujieReact
      width="100%"
      height="100%"
      name={`${connection.daemon?.public_key}/plugin/${pluginName}/${entryLabel}`}
      url={address}
      sync={true}
    />
  );
}
