import {
  Button,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
  useDisclosure,
} from "@nextui-org/react";
import useDaemons from "../../hooks/core/use-daemons";
import { invoke } from "@tauri-apps/api/core";
import useDaemon from "../../hooks/core/use-daemon";
import { RegistedPlugin } from "../../models/core";

interface Props {
  plugin: RegistedPlugin;
}

export default function DeletePluginButton({ plugin }: Props) {
  const { mutate } = useDaemons();
  const isolate = useDaemon();
  const { isOpen, onClose, onOpen } = useDisclosure();

  const handleDelete = async () => {
    await invoke("delete_plugin", {
      publicKey: isolate?.public_key,
      pluginName: plugin.config.name,
    });

    onClose();
    mutate();
  };

  return (
    <>
      <Button color="danger" onClick={onOpen}>
        卸载
      </Button>
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalContent>
          <ModalHeader>正在卸载插件 {plugin.config.name}</ModalHeader>
          <ModalBody>
            卸载插件将移除插件提供的所有服务和数据，确认继续吗？
          </ModalBody>
          <ModalFooter>
            <Button onClick={onClose}>取消</Button>
            <Button onClick={handleDelete} color="danger">
              确认
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
}
