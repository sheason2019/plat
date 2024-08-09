import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/new/import")({
  component: () => <div>Hello /new/import!</div>,
});
