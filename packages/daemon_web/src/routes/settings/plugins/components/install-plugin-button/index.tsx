import { Button } from "@nextui-org/react";
import { FormEvent, useRef } from "react";
import axios from "axios";

export default function InstallPluginButton() {
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSelectFile = async (e: FormEvent<HTMLInputElement>) => {
    const file = e.currentTarget.files?.[0];
    if (!file) return;

    const formData = new FormData();
    formData.set("plugin", file);
    const resp = await axios.post("/api/plugin", formData);
    console.log("response", resp);
  };

  return (
    <div>
      <Button
        color="primary"
        type="button"
        onClick={() => inputRef.current?.click()}
      >
        安装插件
      </Button>
      <input
        className="invisible"
        type="file"
        name="file"
        ref={inputRef}
        onClick={(e) => {
          e.currentTarget.value = "";
        }}
        onInput={handleSelectFile}
      />
    </div>
  );
}
