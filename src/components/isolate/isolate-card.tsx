import { Button, Card, CardBody, CardFooter } from "@nextui-org/react";
import { Isolate } from "../../models/core";
import { Link } from "@tanstack/react-router";

interface Props {
  isolate: Isolate;
}

export default function IsolateCard({ isolate }: Props) {
  const handleClick = () => {
    console.log("click");
  };

  return (
    <Link to="/isolate/$pubkey" params={{ pubkey: isolate.public_key }}>
      <Card>
        <CardBody className="p-2">
          <div className="flex gap-3 items-center">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width={24}
              height={24}
              className="ml-1 text-default-500"
              fill="currentColor"
              viewBox="0 0 16 16"
            >
              <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z" />
              <path
                fillRule="evenodd"
                d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1z"
              />
            </svg>
            <p className="text-default-500 flex-1 overflow-hidden text-ellipsis">
              {isolate.public_key.slice(0, 16)}
            </p>
            <Button
              as="object"
              isIconOnly
              variant="light"
              onClick={(e) => {
                e.preventDefault();
                handleClick();
              }}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width={24}
                height={24}
                fill="currentColor"
                viewBox="0 0 16 16"
              >
                <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z" />
                <path
                  fillRule="evenodd"
                  d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1z"
                />
              </svg>
            </Button>
          </div>
        </CardBody>
      </Card>
    </Link>
  );
}
