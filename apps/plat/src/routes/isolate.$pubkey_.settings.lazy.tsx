import { createLazyFileRoute, Outlet } from "@tanstack/react-router";
import Header from "../components/header";
import { Listbox, ListboxItem } from "@nextui-org/react";

export const Route = createLazyFileRoute("/isolate/$pubkey/settings")({
  component: Settings,
});

function Settings() {
  const { pubkey } = Route.useParams();

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <Header title="用户设置" backHref={`/isolate/${pubkey}`} />
      <div className="flex flex-1 overflow-hidden items-stretch">
        <Listbox className="max-w-xs">
          <ListboxItem key="plugin" href={`/isolate/${pubkey}/settings/plugin`}>
            管理插件
          </ListboxItem>
        </Listbox>
        <div className="flex-1 overflow-y-auto">
          <Outlet />
        </div>
      </div>
    </div>
  );
}
