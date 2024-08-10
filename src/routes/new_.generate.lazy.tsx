import { createLazyFileRoute } from "@tanstack/react-router";
import { Button, Select, SelectItem } from "@nextui-org/react";
import Header from "../components/header";
import { invoke } from "@tauri-apps/api/core";

export const Route = createLazyFileRoute("/new/generate")({
  component: Import,
});

function Import() {
  const navigate = Route.useNavigate();
  const handleGenerate = async () => {
    await invoke("create_isolate", { template: "default" });
    navigate({
      to: "/",
    });
  };

  return (
    <>
      <Header title="生成账号" backHref="/new" />
      <div className="container max-w-xs mx-auto px-2 flex-1">
        <Select
          label="选择账号模板"
          placeholder="请选择一个账号模板"
          className="mt-3"
        >
          <SelectItem key="default">default</SelectItem>
        </Select>
      </div>
      <div className="container max-w-xs mx-auto px-2 mb-2">
        <Button
          className="w-full mt-4"
          color="primary"
          onClick={handleGenerate}
        >
          生成账号
        </Button>
      </div>
    </>
  );
}
