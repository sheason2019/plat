import { useMemo } from "react";
import useStage from "./hooks/use-stage";

export default function InnerStage() {
  const [stage] = useStage();
  const url = useMemo(() => {
    if (stage?.plugin.addr) {
      return `http://${stage.plugin.addr}${stage.entry.href}`;
    } else {
      return null;
    }
  }, [stage]);

  console.log('url', url);

  if (!url) return null;

  return <iframe className="w-full h-full overflow-hidden ml-2" src={url} />;
}
