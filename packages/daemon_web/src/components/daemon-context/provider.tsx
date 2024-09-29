import { PropsWithChildren, useEffect, useState } from "react";
import { daemonContext, IDaemonContext } from "./context";
import { Spinner } from "@nextui-org/react";

const { Provider } = daemonContext;

export default function DaemonContextProvider({ children }: PropsWithChildren) {
  const [pending, setPending] = useState(true);
  const [state, setState] = useState<IDaemonContext | null>(null);

  const handleReceiveContext = (context: IDaemonContext) => {
    setState(context);
    window.parent.postMessage({ type: "context-received" }, context.fromOrigin);
  };

  useEffect(() => {
    const listener = (e: MessageEvent) => {
      const data = JSON.parse(e.data);
      switch (data.type) {
        case "context":
          return handleReceiveContext(data.payload);
        default:
          break;
      }
    };
    window.addEventListener("message", listener);
    return () => window.removeEventListener("message", listener);
  }, []);

  useEffect(() => {
    const timeout = setTimeout(() => {
      setPending(false);
    }, 500);

    return () => clearTimeout(timeout);
  }, []);

  const isPending = state === null && pending;

  if (isPending) {
    return (
      <div className="h-full flex flex-col justify-center items-center">
        <Spinner label="正在初始化" labelColor="primary" />
      </div>
    );
  }

  return <Provider value={state}>{children}</Provider>;
}
