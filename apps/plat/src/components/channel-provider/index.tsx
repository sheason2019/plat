import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { PropsWithChildren, useEffect } from "react";
import { useSetRecoilState } from "recoil";
import ConfirmSignature, { confirmSignatureAtom } from "./confirm-signature";
import ConfirmInstallPlugin, {
  confirmInstallPluginAtom,
} from "./confirm-install-plugin";

interface ChannelData {
  id: string;
  type: string;
  data: any;
}

export default function ChannelProvider(props: PropsWithChildren) {
  const setConfirmSignatureState = useSetRecoilState(confirmSignatureAtom);
  const setConfirmInstallPluginState = useSetRecoilState(
    confirmInstallPluginAtom
  );

  useEffect(() => {
    let close = false;
    let unlisten: UnlistenFn | undefined;

    listen<ChannelData>("channel", (e) => {
      switch (e.payload.type) {
        case "confirm-sign":
          setConfirmSignatureState({
            id: e.payload.id,
            data: e.payload.data,
          });
          break;
        case "confirm-install-plugin":
          setConfirmInstallPluginState({
            id: e.payload.id,
            data: e.payload.data,
          });
          break;
      }
    }).then((unlistenFn) => {
      if (close) {
        unlistenFn();
      } else {
        unlisten = unlistenFn;
      }
    });

    return () => {
      close = true;
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  return (
    <>
      <ConfirmSignature />
      <ConfirmInstallPlugin />
      {props.children}
    </>
  );
}
