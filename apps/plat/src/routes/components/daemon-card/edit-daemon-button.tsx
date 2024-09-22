import {
  Button,
  Input,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
  useDisclosure,
} from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/core";
import useDaemonScopes from "../../../hooks/use-daemons";

interface Props {
  daemonKey: string;
}

export default function EditDaemonButton({ daemonKey }: Props) {
  const { mutate } = useDaemonScopes();
  const { isOpen, onClose, onOpen } = useDisclosure();

  const handleSubmit = async (formData: FormData) => {
    await invoke("update_daemon_password", {
      daemonKey,
      newPassword: formData.get("password")?.toString(),
    });
    mutate();
    onClose();
  };

  return (
    <>
      <Button isIconOnly onClick={onOpen} variant="light">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          fill="currentColor"
          viewBox="0 0 16 16"
        >
          <path d="M15.502 1.94a.5.5 0 0 1 0 .706L14.459 3.69l-2-2L13.502.646a.5.5 0 0 1 .707 0l1.293 1.293zm-1.75 2.456-2-2L4.939 9.21a.5.5 0 0 0-.121.196l-.805 2.414a.25.25 0 0 0 .316.316l2.414-.805a.5.5 0 0 0 .196-.12l6.813-6.814z" />
          <path
            fillRule="evenodd"
            d="M1 13.5A1.5 1.5 0 0 0 2.5 15h11a1.5 1.5 0 0 0 1.5-1.5v-6a.5.5 0 0 0-1 0v6a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5v-11a.5.5 0 0 1 .5-.5H9a.5.5 0 0 0 0-1H2.5A1.5 1.5 0 0 0 1 2.5v11z"
          />
        </svg>
      </Button>
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalContent>
          <form
            onSubmit={async (e) => {
              e.preventDefault();
              await handleSubmit(new FormData(e.currentTarget));
            }}
          >
            <ModalHeader>修改密码</ModalHeader>
            <ModalBody>
              <Input
                name="password"
                placeholder="输入新的密码"
                type="password"
              />
            </ModalBody>
            <ModalFooter>
              <Button color="primary" type="submit">
                修改
              </Button>
            </ModalFooter>
          </form>
        </ModalContent>
      </Modal>
    </>
  );
}
