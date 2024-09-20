import { Button } from "@nextui-org/react";

export default function ConnectionFailed() {
  return (
    <div className="h-full-parent h-full flex flex-col items-center justify-center gap-3">
      <p className="text-red-600">网络连接已中断</p>
      <Button color="primary" onClick={() => location.reload()}>
        重新连接
      </Button>
    </div>
  );
}
