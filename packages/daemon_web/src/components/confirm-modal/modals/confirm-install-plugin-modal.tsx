import {
  Button,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
} from "@nextui-org/react";
import useConfirmModal from "../hooks/use-confirm-modal";
import { ConfirmModalVariant } from "../typings";
import { useMemo } from "react";
import { useRecoilValue } from "recoil";
import { connectionState } from "../../connection-provider/context";

export default function ConfirmInstallPluginModal() {
  const connection = useRecoilValue(connectionState);
  const { state, closeModal } = useConfirmModal();

  const open = state?.variant === ConfirmModalVariant.InstallPlugin;

  const plugin = useMemo(() => {
    if (open) return state.plugin;
    return null;
  }, [state, open]);

  const name = useMemo(() => {
    if (open) return state.name;
    return null;
  }, [state, open]);

  const handleResult = (allow: boolean) => {
    connection.ws?.send(
      JSON.stringify({
        type: "confirm/install-plugin",
        payload: {
          name,
          allow,
        },
      })
    );
    closeModal();
  };

  const handleOk = () => {
    handleResult(true);
  };
  const handleCancel = () => {
    handleResult(false);
  };

  return (
    <Modal isOpen={open}>
      <ModalContent>
        <ModalHeader>安装插件</ModalHeader>
        <ModalBody>
          <pre className="break-words">
            {JSON.stringify(plugin, null, "  ")}
          </pre>
        </ModalBody>
        <ModalFooter>
          <Button onClick={handleCancel}>取消</Button>
          <Button color="primary" onClick={handleOk}>
            确认安装
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}
