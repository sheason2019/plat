import { createLazyFileRoute } from "@tanstack/react-router";
import IsolateDrawer from "../components/isolate-drawer";
import Stage from "../components/stage";

export const Route = createLazyFileRoute("/isolate/$pubkey")({
  component: Isolate,
});

function Isolate() {
  return (
    <div className="flex-1 flex items-stretch">
      <IsolateDrawer />
      <Stage />
    </div>
  );
}
