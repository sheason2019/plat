import { createLazyFileRoute } from "@tanstack/react-router";
import IsolateDrawer from "../components/isolate-drawer";

export const Route = createLazyFileRoute("/isolate/$pubkey")({
  component: Isolate,
});

function Isolate() {
  return (
    <div className="flex-1 flex items-stretch">
      <IsolateDrawer />
    </div>
  );
}
