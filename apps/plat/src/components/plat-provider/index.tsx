import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { PropsWithChildren, useEffect, useRef } from "react";
import useDaemonScopes from "../../hooks/use-daemons";

export default function PlatProvider({ children }: PropsWithChildren) {
  const { mutate } = useDaemonScopes();
  const unlistenRef = useRef<UnlistenFn | null>(null);

  useEffect(() => {
    listen("update-daemons", () => {
      mutate();
    }).then((unlisten) => {
      if (unlistenRef.current !== null) {
        unlistenRef.current();
        unlistenRef.current = null;
      }

      unlistenRef.current = unlisten;
    });
  }, []);

  return <>{children}</>;
}
