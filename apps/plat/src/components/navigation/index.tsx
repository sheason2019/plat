import { Button, Link } from "@nextui-org/react";
import { Drawer } from "vaul";

export default function Navigation() {
  <Drawer.Root>
    <Button isIconOnly variant="light" as={Drawer.Trigger}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        fill="currentColor"
        viewBox="0 0 16 16"
      >
        <path
          fillRule="evenodd"
          d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"
        />
      </svg>
    </Button>
    <Drawer.Portal>
      <Drawer.Content>
        <Drawer.Title>Title</Drawer.Title>
        <p>Hello world</p>
      </Drawer.Content>
      <Drawer.Overlay />
    </Drawer.Portal>
  </Drawer.Root>;

  return (
    <Drawer.Root direction="left">
      <Button isIconOnly variant="light" as={Drawer.Trigger}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          fill="currentColor"
          viewBox="0 0 16 16"
        >
          <path
            fillRule="evenodd"
            d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"
          />
        </svg>
      </Button>
      <Drawer.Portal>
        <Drawer.Overlay className="fixed inset-0 bg-black/40" />
        <Drawer.Content
          className="left-0 top-0 bottom-0 fixed z-10 flex outline-none"
          aria-describedby=""
        >
          <div className="bg-zinc-50 rounded-[16px] w-[310px] grow mt-2 ml-2 mb-2 p-5 flex flex-col">
            <div className="max-w-md">
              <Drawer.Title className="font-medium mb-3 text-zinc-900">
                导航
              </Drawer.Title>
              <Button
                as={Link}
                href="/templates"
                className="w-full justify-start"
                variant="light"
              >
                Daemon 模板
              </Button>
            </div>
          </div>
        </Drawer.Content>
      </Drawer.Portal>
    </Drawer.Root>
  );
}
