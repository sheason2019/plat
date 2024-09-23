import { Button } from "@nextui-org/react";
import { useRef } from "react";
import { useSetRecoilState } from "recoil";
import { erudaAtom } from "../../../components/eruda-provider/atom";

interface Props {
  onOpen(): void;
}

export default function CreateDaemonCard({ onOpen }: Props) {
  const setEruda = useSetRecoilState(erudaAtom);
  const timeoutRef = useRef<number | null>(null);

  const onPressStart = () => {
    timeoutRef.current = setTimeout(() => {
      setEruda((prev) => ({ ...prev, show: !prev.show }));
    }, 5000);
  };
  const onPressEnd = () => {
    timeoutRef.current && clearTimeout(timeoutRef.current);
  };

  return (
    <Button
      isIconOnly
      onClick={onOpen}
      onPressStart={onPressStart}
      onPressEnd={onPressEnd}
      size="lg"
      color="secondary"
      className="fixed right-4 bottom-4"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width={20}
        height={20}
        fill="currentColor"
        viewBox="0 0 16 16"
      >
        <path
          fillRule="evenodd"
          d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2Z"
        />
      </svg>
    </Button>
  );
}
