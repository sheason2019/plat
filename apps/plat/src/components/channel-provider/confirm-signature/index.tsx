import { atom, useRecoilState } from "recoil";
import { ConfirmSignatureAtom } from "./typings";
import {
  Button,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
} from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/core";

export const confirmSignatureAtom = atom<ConfirmSignatureAtom | null>({
  key: "confirm-sign",
  default: null,
});

export default function ConfirmSignature() {
  const [state, setState] = useRecoilState(confirmSignatureAtom);

  const handleConfirm = async (allow: boolean) => {
    invoke("channel", { id: state?.id, data: JSON.stringify({ allow }) });
    setState(null);
  };

  return (
    <Modal
      isOpen={!!state}
      placement="bottom-center"
      scrollBehavior="inside"
      backdrop="blur"
      isDismissable={false}
      isKeyboardDismissDisabled={false}
      onClose={() => handleConfirm(false)}
    >
      <ModalContent>
        <ModalHeader>数字签名请求</ModalHeader>
        <ModalBody>
          <div>
            <p className="font-bold">请求 ID</p>
            <code className="break-words break-all">{state?.id}</code>
          </div>
          {!!state?.data.describe.length && (
            <div>
              <p className="font-bold">签名描述</p>
              <code className="break-words break-all">
                {state?.data.describe}
              </code>
            </div>
          )}
          <div>
            <p className="font-bold">签名内容</p>
            <code className="break-words break-all">{state?.data.data}</code>
          </div>
        </ModalBody>
        <ModalFooter>
          <Button color="primary" onClick={() => handleConfirm(true)}>
            确认签名
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}
