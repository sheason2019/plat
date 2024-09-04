import {
  Button,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
  useDisclosure,
} from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/core";
import useProfile from "../../hooks/core/use-profile";

interface Props {
  publicKey: string;
  onClose(): void;
}

export default function DeleteIsolateButton({
  publicKey,
  onClose: onCloseParent,
}: Props) {
  const { mutate } = useProfile();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const handleDelete = async () => {
    await invoke("delete_isolate", { publicKey });
    mutate();
    onClose();
    onCloseParent();
  };

  return (
    <>
      <Button color="danger" onClick={onOpen}>
        删除账号
      </Button>
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalContent>
          <ModalHeader>警告</ModalHeader>
          <ModalBody>
            该操作将导致账号信息在当前设备上永久删除，请确认是否继续执行删除操作！
          </ModalBody>
          <ModalFooter>
            <Button onClick={onClose}>取消</Button>
            <Button color="danger" onClick={handleDelete}>
              确认删除
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
}
