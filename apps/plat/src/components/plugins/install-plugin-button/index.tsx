import {
  Button,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownTrigger,
} from "@nextui-org/react";
import { open } from "@tauri-apps/plugin-dialog";
import useIsolate from "../../../hooks/core/use-isolate";
import useProfile from "../../../hooks/core/use-profile";
import { invoke } from "@tauri-apps/api/core";

export default function InstallPluginButton() {
  const { mutate } = useProfile();
  const isolate = useIsolate();

  const handleInstallFromFile = async () => {
    const file = await open({
      filters: [{ name: "plat-extension", extensions: ["plat"] }],
    });

    if (!file) {
      return;
    }

    await invoke("install_plugin", {
      publicKey: isolate?.public_key,
      pluginFilePath: file,
    });

    mutate();
  };

  return (
    <Dropdown>
      <DropdownTrigger>
        <Button
          color="primary"
          endContent={
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              fill="currentColor"
              viewBox="0 0 16 16"
            >
              <path d="M7.247 11.14 2.451 5.658C1.885 5.013 2.345 4 3.204 4h9.592a1 1 0 0 1 .753 1.659l-4.796 5.48a1 1 0 0 1-1.506 0z" />
            </svg>
          }
        >
          添加插件
        </Button>
      </DropdownTrigger>
      <DropdownMenu aria-label="install plugin options" disabledKeys={["url"]}>
        <DropdownItem key="file" onClick={handleInstallFromFile}>
          通过文件安装
        </DropdownItem>
        <DropdownItem key="url">通过Url安装</DropdownItem>
      </DropdownMenu>
    </Dropdown>
  );
}
