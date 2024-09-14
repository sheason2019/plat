import { createLazyFileRoute } from "@tanstack/react-router";
import Stage from "../components/stage";
import DaemonControl from "../components/daemon-control";

export const Route = createLazyFileRoute("/isolate/$pubkey")({
  component: DaemonPage,
});

function DaemonPage() {
  return (
    <div className="flex-1 flex items-stretch relative">
      <DaemonControl />
      <Stage />
    </div>
  );
}
