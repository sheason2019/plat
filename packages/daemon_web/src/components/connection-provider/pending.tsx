import { Spinner } from "@nextui-org/react";

export default function ConnectionPending() {
  return (
    <div className="h-full-parent h-full flex flex-col items-center justify-center">
      <Spinner label="正在连接" labelColor="primary" />
    </div>
  );
}
