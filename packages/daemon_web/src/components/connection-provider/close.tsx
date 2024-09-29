import { Button } from "@nextui-org/react";

interface Props {
  reason: string;
}

export default function ConnectionClose({ reason }: Props) {
  return (
    <div className="h-full flex flex-col items-center justify-center gap-3">
      <p className="text-red-600">连接已断开</p>
      <p className="text-sm">原因：{reason}</p>
      <Button
        color="primary"
        className="mt-3"
        onClick={() => location.reload()}
      >
        重新连接
      </Button>
    </div>
  );
}
