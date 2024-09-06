import { Card, CardBody, Modal, useDisclosure } from "@nextui-org/react";
import { Daemon } from "../../models/core";
import { Link } from "@tanstack/react-router";
import IsolateCardModalContent from "./isolate-card-modal-content";

interface Props {
  isolate: Daemon;
}

export default function IsolateCard({ isolate }: Props) {
  const { isOpen, onOpen, onClose } = useDisclosure();

  return (
    <div>
      <Link to="/isolate/$pubkey" params={{ pubkey: isolate.public_key }} />
      <Card>
        <CardBody
          className="px-2 py-0 cursor-pointer hover:bg-default-100"
          onClick={onOpen}
        >
          <div className="flex gap-3 items-center h-12">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width={24}
              height={24}
              className="ml-1 text-default-500"
              fill="currentColor"
              viewBox="0 0 16 16"
            >
              <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z" />
              <path
                fillRule="evenodd"
                d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1z"
              />
            </svg>
            <p className="text-default-500 flex-1 overflow-hidden text-ellipsis">
              {isolate.public_key.slice(0, 16)}
            </p>
          </div>
        </CardBody>
      </Card>
      <Modal isOpen={isOpen} onClose={onClose}>
        <IsolateCardModalContent isolate={isolate} onClose={onClose} />
      </Modal>
    </div>
  );
}
