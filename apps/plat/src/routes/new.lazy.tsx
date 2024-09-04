import { createLazyFileRoute, Link } from "@tanstack/react-router";
import Header from "../components/header";
import { Card, CardBody } from "@nextui-org/react";

export const Route = createLazyFileRoute("/new")({
  component: New,
});

function New() {
  return (
    <>
      <Header backHref="/" title="生成 / 导入账号" />
      <div className="flex-1 flex justify-center items-center">
        <div className="max-w-xs w-full flex flex-col gap-3">
          <Card as={Link} to="/new/generate">
            <CardBody className="text-center text-default-500 text-sm">
              生成
            </CardBody>
          </Card>
          <Card as={Link} to="/new/import">
            <CardBody className="text-center text-default-500 text-sm">
              导入
            </CardBody>
          </Card>
        </div>
      </div>
    </>
  );
}
