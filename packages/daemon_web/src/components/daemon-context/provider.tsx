import { PropsWithChildren, useEffect, useState } from "react";
import { daemonContext, IDaemonContext } from "./context";

const { Provider } = daemonContext;

export default function DaemonContextProvider({ children }: PropsWithChildren) {
  const [state, setState] = useState<IDaemonContext | null>(null);

  useEffect(() => {
    const listener = (e: MessageEvent) => {
      const data = JSON.parse(e.data);
      switch (data.type) {
        case "context":
          setState(data.payload);
          break;
        default:
          break;
      }
    };
    window.addEventListener("message", listener);
    return () => window.removeEventListener("message", listener);
  }, []);

  return <Provider value={state}>{children}</Provider>;
}
