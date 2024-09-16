import {
  Button,
  Link,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
} from "@nextui-org/react";
import { Daemon } from "../../../models/core";
import DeleteIsolateButton from "./delete-isolate-button";

interface Props {
  daemon: Daemon;
  onClose(): void;
}

export default function IsolateCardModalContent({ daemon, onClose }: Props) {
  return (
    <ModalContent>
      <ModalHeader>账号信息</ModalHeader>
      <ModalBody>
        <div>
          <p>公钥</p>
          <p className="text-sm text-default-500 whitespace-nowrap text-ellipsis">
            {daemon.public_key}
          </p>
        </div>
      </ModalBody>
      <ModalFooter>
        <DeleteIsolateButton publicKey={daemon.public_key} onClose={onClose} />
        <div className="flex-1" />
        <Button
          color="primary"
          as={Link}
          href={"/isolate/" + daemon.public_key}
        >
          进入账号主页面
        </Button>
      </ModalFooter>
    </ModalContent>
  );
}
