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

interface Props {
  daemonKey: string;
}

export default function DeleteDaemonButton({ daemonKey }: Props) {
  const { isOpen, onOpen, onClose } = useDisclosure();

  const handleDelete = async () => {
    await invoke("remove_daemon", { daemonKey });
    onClose();
  };

  return (
    <>
      <Button color="danger" variant="light" isIconOnly onClick={onOpen}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          fill="currentColor"
          viewBox="0 0 16 16"
        >
          <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z" />
          <path
            fillRule="evenodd"
            d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"
          />
        </svg>
      </Button>
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalContent>
          <ModalHeader>警告</ModalHeader>
          <ModalBody>
            即将在本设备上删除此账号的数据，未经备份的账号将永久无法找回，确认要执行删除操作吗？
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
