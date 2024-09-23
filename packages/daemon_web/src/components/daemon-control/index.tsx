import { useDisclosure } from "@nextui-org/react";
import DaemonControlButton from "./button";
import DaemonControlModal from "./modal";

export default function DaemonControl() {
  const { isOpen, onClose, onOpen } = useDisclosure();

  return (
    <>
      <DaemonControlButton onClick={onOpen} />
      <DaemonControlModal isOpen={isOpen} onClose={onClose} />
    </>
  );
}
