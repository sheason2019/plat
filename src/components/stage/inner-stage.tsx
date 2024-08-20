import { useMemo } from "react";
import useStage from "./hooks/use-stage";

export default function InnerStage() {
  const [stage] = useStage();
  const url = useMemo(() => {
    if (stage?.plugin.port) {
      return `http://127.0.0.1:${stage.plugin.port}${stage.entry.href}`;
    } else {
      return null;
    }
  }, [stage]);

  if (!url) return null;

  return <iframe className="w-full h-full overflow-hidden ml-2" src={url} />;
}
