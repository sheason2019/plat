import {
  Button,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
} from "@nextui-org/react";
import { Isolate } from "../../models/core";
import DeleteIsolateButton from "./delete-isolate-button";
import { Link } from "@tanstack/react-router";

interface Props {
  isolate: Isolate;
  onClose(): void;
}

export default function IsolateCardModalContent({ isolate, onClose }: Props) {
  return (
    <ModalContent>
      <ModalHeader>账号信息</ModalHeader>
      <ModalBody>
        <div>
          <p>公钥</p>
          <p className="text-sm text-default-500 whitespace-nowrap text-ellipsis">
            {isolate.public_key}
          </p>
        </div>
      </ModalBody>
      <ModalFooter>
        <DeleteIsolateButton publicKey={isolate.public_key} onClose={onClose} />
        <div className="flex-1" />
        <Button
          color="primary"
          as={Link}
          to="/isolate/$pubkey"
          params={{ pubkey: isolate.public_key }}
        >
          进入账号主页面
        </Button>
      </ModalFooter>
    </ModalContent>
  );
}
