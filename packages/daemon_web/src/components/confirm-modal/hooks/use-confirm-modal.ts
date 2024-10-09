import { atom, useRecoilState } from "recoil";
import { IPlugin } from "../../connection-provider/typings";
import { ConfirmModalState, ConfirmModalVariant } from "../typings";
import { useCallback } from "react";

const confirmModalVariantState = atom<ConfirmModalState>({
  key: "confirmModalVariant",
  default: null,
});

export default function useConfirmModal() {
  const [state, setState] = useRecoilState(confirmModalVariantState);

  const confirmInstallPlugin = useCallback(
    (name: string, plugin: IPlugin) => {
      setState({
        variant: ConfirmModalVariant.InstallPlugin,
        plugin,
        name,
      });
    },
    [setState]
  );

  const confirmRemovePlugin = useCallback(
    (name: string) => {
      setState({
        variant: ConfirmModalVariant.RemovePlugin,
        name,
      });
    },
    [setState]
  );

  const closeModal = () => setState(null);

  return { confirmInstallPlugin, confirmRemovePlugin, state, closeModal };
}
