import { createLazyFileRoute } from "@tanstack/react-router";
import Header from "../components/header";

export const Route = createLazyFileRoute("/new/generate")({
  component: Import,
});

function Import() {
  return (
    <>
      <Header title="生成账号" backHref="/new" />
    </>
  );
}
