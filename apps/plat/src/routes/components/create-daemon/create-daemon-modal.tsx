import {
  Modal,
  ModalContent,
  ModalHeader,
  ModalBody,
  Select,
  SelectItem,
  ModalFooter,
  Button,
  Input,
} from "@nextui-org/react";
import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

interface Props {
  isOpen: boolean;
  onClose(): void;
}

enum Variant {
  Null = "",
  LocalGenerate = "local-generate",
  Remote = "remote",
}

interface CreateDaemonData {
  variant: Variant;
  remoteAddress: string;
}

const DEFAULT_VALUE: CreateDaemonData = {
  variant: Variant.Null,
  remoteAddress: "",
} as const;

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
            value={form.variant}
            onChange={(e) =>
              setForm((prev) => ({
                ...prev,
                variant: e.target.value as Variant,
              }))
            }
          >
            <SelectItem key={Variant.LocalGenerate}>本地生成</SelectItem>
            <SelectItem key={Variant.Remote}>远程服务</SelectItem>
          </Select>
          {form.variant === Variant.Remote && (
            <>
              <Input
                label="远程服务地址"
                value={form.remoteAddress}
                onChange={(e) =>
                  setForm((prev) => ({
                    ...prev,
                    remoteAddress: e.target.value,
                  }))
                }
              />
            </>
          )}
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
