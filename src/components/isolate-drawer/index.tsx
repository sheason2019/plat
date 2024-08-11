import { Button, Card, CardBody, useDisclosure } from "@nextui-org/react";
import { Link } from "@tanstack/react-router";
import clsx from "clsx";

export default function IsolateDrawer() {
  const { isOpen, onOpenChange } = useDisclosure();

  return (
    <Card
      className={clsx("bg-blue-50 rounded-l-none", isOpen ? "w-full" : "w-20")}
    >
      <CardBody className="py-2 px-0">
        <div className="flex items-stretch flex-1">
          {isOpen && (
            <div className="flex-1">
              <Button
                isIconOnly
                className="w-16 h-16"
                variant="light"
                as={Link}
                to="/"
              >
                选择账号
              </Button>
            </div>
          )}
          <div className="flex flex-col h-full items-center w-16 mx-2 shrink-0">
            <div className="flex-1 flex flex-col items-center">
              <Button isIconOnly className="w-16 h-16" variant="light">
                App1
              </Button>
            </div>
            <div>
              <Button isIconOnly size="lg" onClick={onOpenChange}></Button>
            </div>
          </div>
        </div>
      </CardBody>
    </Card>
  );
}
