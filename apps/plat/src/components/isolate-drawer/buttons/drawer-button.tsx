import { Button, ButtonProps } from "@nextui-org/react";
import clsx from "clsx";

export default function DrawerButton(props: ButtonProps) {
  return (
    <Button
      isIconOnly
      variant="flat"
      {...props}
      className={clsx("w-16 h-16", props.className)}
    />
  );
}
