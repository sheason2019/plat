import { atom, useRecoilState } from "recoil";
import { ConfirmInstallPluginAtom } from "./typings";
import {
  Button,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
} from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/core";

export const confirmInstallPluginAtom = atom<ConfirmInstallPluginAtom | null>({
  key: "confirm-install-plugin",
  default: null,
});

export default function ConfirmInstallPlugin() {
  const [state, setState] = useRecoilState(confirmInstallPluginAtom);

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
        <ModalHeader>插件安装请求</ModalHeader>
        <ModalBody>
          <div>
            <p className="font-bold">请求 ID</p>
            <code className="break-words break-all">{state?.id}</code>
          </div>
          <div>
            <p className="font-bold">插件配置文件</p>
            <code className="break-words break-all">
              {JSON.stringify(state?.data)}
            </code>
          </div>
        </ModalBody>
        <ModalFooter>
          <Button color="primary" onClick={() => handleConfirm(true)}>
            确认安装
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}
