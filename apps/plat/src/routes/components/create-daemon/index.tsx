import { useDisclosure } from "@nextui-org/react";
import CreateDaemonModal from "./create-daemon-modal";
import CreateDaemonFab from "./create-daemon-fab";

export default function CreateDaemon() {
  const { isOpen, onClose, onOpen } = useDisclosure();

  return (
    <>
      <CreateDaemonFab onOpen={onOpen} />
      <CreateDaemonModal isOpen={isOpen} onClose={onClose} />
    </>
  );
}
