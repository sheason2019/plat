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

export default function ConfirmDeletePluginModal() {
  const connection = useRecoilValue(connectionState);
  const { state, closeModal } = useConfirmModal();
  const open = state?.variant === ConfirmModalVariant.DeletePlugin;

  const plugin = useMemo(() => {
    if (open) return state.plugin;
    return null;
  }, [open, state]);

  const handleResult = (allow: boolean) => {
    connection.ws?.send(
      JSON.stringify({
        type: "confirm/delete-plugin",
        payload: {
          name: plugin?.name,
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
        <ModalHeader>删除插件</ModalHeader>
        <ModalBody>
          <pre>{JSON.stringify(plugin, null, "  ")}</pre>
        </ModalBody>
        <ModalFooter>
          <Button onClick={handleCancel}>取消</Button>
          <Button onClick={handleOk} color="danger">
            确认删除
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}
