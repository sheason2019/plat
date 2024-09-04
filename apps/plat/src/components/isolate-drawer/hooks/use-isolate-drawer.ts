import { atom, useRecoilState } from "recoil";

const drawerState = atom({
  key: "isolate-drawer",
  default: false,
});

export default function useIsolateDrawer() {
  const [isOpen, setIsOpen] = useRecoilState(drawerState);

  const onClose = () => setIsOpen(false);
  const onOpen = () => setIsOpen(true);

  const onOpenChange = () => setIsOpen(!isOpen);

  return {
    isOpen,
    onClose,
    onOpen,
    onOpenChange,
  };
}
