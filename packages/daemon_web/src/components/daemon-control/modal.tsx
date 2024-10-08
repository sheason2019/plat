import {
  Button,
  Divider,
  Link,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
} from "@nextui-org/react";
import { connectionState } from "../connection-provider/context";
import { useRecoilValue } from "recoil";
import PluginEntry from "./plugin-entry";
import { Fragment } from "react/jsx-runtime";

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

export default function DaemonControlModal({ isOpen, onClose }: Props) {
  const connection = useRecoilValue(connectionState);

  const handleExit = async () => {
    window.parent.postMessage({ type: "exit" }, "*");
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalContent>
        <ModalHeader>
          <div className="overflow-hidden">
            <p>菜单</p>
            <div className="text-sm text-gray-500 whitespace-nowrap overflow-hidden text-ellipsis">
              <p>账号公钥：{connection?.daemon.public_key}</p>
              <p>网络地址：{location.origin}</p>
            </div>
          </div>
        </ModalHeader>
        <ModalBody>
          <div className="grid grid-cols-3 mb-4">
            {connection?.daemon.plugins.map((plugin) => (
              <Fragment key={plugin.name}>
                {plugin.entries.map((entry) => (
                  <PluginEntry
                    key={entry.label}
                    plugin={plugin}
                    entry={entry}
                    onClose={onClose}
                  />
                ))}
              </Fragment>
            ))}
          </div>
        </ModalBody>
        <Divider />
        <ModalFooter>
          <Button
            startContent={
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                fill="currentColor"
                viewBox="0 0 16 16"
              >
                <path d="M8.538 1.02a.5.5 0 1 0-.076.998 6 6 0 1 1-6.445 6.444.5.5 0 0 0-.997.076A7 7 0 1 0 8.538 1.02Z" />
                <path d="M7.096 7.828a.5.5 0 0 0 .707-.707L2.707 2.025h2.768a.5.5 0 1 0 0-1H1.5a.5.5 0 0 0-.5.5V5.5a.5.5 0 0 0 1 0V2.732l5.096 5.096Z" />
              </svg>
            }
            onClick={handleExit}
            color="danger"
          >
            退出账号
          </Button>
          <div className="flex-1" />
          <Button
            startContent={
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width={20}
                height={20}
                fill="currentColor"
                viewBox="0 0 16 16"
              >
                <path d="M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 0 1-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 0 1 .872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 0 1 2.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 0 1 2.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 0 1 .872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 0 1-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 0 1-2.105-.872l-.1-.34zM8 10.93a2.929 2.929 0 1 1 0-5.86 2.929 2.929 0 0 1 0 5.858z" />
              </svg>
            }
            color="primary"
            as={Link}
            href="/settings"
            onClick={onClose}
          >
            设置
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}
