import { Button, Input, Spinner } from "@nextui-org/react";
import { useMemo } from "react";

interface Props {
  inputPassword: ((pswd: string) => void) | null;
}

export default function ConnectionPending({ inputPassword }: Props) {
  const content = useMemo(() => {
    if (inputPassword) {
      return (
        <form
          className="max-w-xs w-full flex flex-col items-center gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            const formData = new FormData(e.currentTarget);
            const value = formData.get("password")?.toString() ?? "";
            inputPassword(value);
          }}
        >
          <p className="text-xl font-bold">账号登录</p>
          <Input name="password" type="password" label="账户密码" />
          <Button color="primary" type="submit" className="px-12">
            登录
          </Button>
        </form>
      );
    } else {
      return (
        <div>
          <Spinner label="正在连接" labelColor="primary" />
        </div>
      );
    }
  }, [inputPassword]);

  return (
    <div className="h-full-parent h-full flex flex-col items-center justify-center">
      {content}
    </div>
  );
}
