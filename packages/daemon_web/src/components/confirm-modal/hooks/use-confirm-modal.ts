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

  const confirmDeletePlugin = useCallback(
    (plugin: IPlugin) => {
      setState({
        variant: ConfirmModalVariant.DeletePlugin,
        plugin,
      });
    },
    [setState]
  );

  const closeModal = () => setState(null);

  return { confirmInstallPlugin, confirmDeletePlugin, state, closeModal };
}
