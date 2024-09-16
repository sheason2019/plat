import {
  Modal,
  ModalContent,
  ModalHeader,
  ModalBody,
  Select,
  SelectItem,
  ModalFooter,
  Button,
} from "@nextui-org/react";
import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

interface Props {
  isOpen: boolean;
  onClose(): void;
}

interface CreateDaemonData {
  variant: string;
}

const DEFAULT_VALUE: CreateDaemonData = { variant: "" } as const;

export default function CreateDaemonModal({ isOpen, onClose }: Props) {
  const [form, setForm] = useState(DEFAULT_VALUE);

  useEffect(() => {
    setForm(DEFAULT_VALUE);
  }, [isOpen]);

  const handleSubmit = async () => {
    await invoke("append_daemon", form as unknown as InvokeArgs);
    onClose();
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalContent>
        <ModalHeader>添加账号</ModalHeader>
        <ModalBody>
          <Select
            name="variant"
            label="添加账号的方式"
            onChange={(e) =>
              setForm((prev) => ({ ...prev, variant: e.target.value }))
            }
          >
            <SelectItem key="local-generate">本地生成</SelectItem>
          </Select>
        </ModalBody>
        <ModalFooter>
          <Button
            color="primary"
            isDisabled={form.variant.length === 0}
            onClick={handleSubmit}
          >
            提交
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}
