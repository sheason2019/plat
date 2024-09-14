import { ReactNode } from "react";
import clsx from "clsx";
import { Button, Link } from "@nextui-org/react";

interface Props {
  title: ReactNode;
  backHref?: string;
}

export default function Header({ title, backHref }: Props) {
  return (
    <div
      className={clsx(
        "backdrop-blur-md sticky top-0",
        "flex items-center px-3 my-1 h-10"
      )}
    >
      {backHref && (
        <Button
          isIconOnly
          variant="light"
          className="mr-2"
          as={Link}
          href={backHref}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width={28}
            height={28}
            fill="currentColor"
            viewBox="0 0 16 16"
          >
            <path
              fillRule="evenodd"
              d="M12 8a.5.5 0 0 1-.5.5H5.707l2.147 2.146a.5.5 0 0 1-.708.708l-3-3a.5.5 0 0 1 0-.708l3-3a.5.5 0 1 1 .708.708L5.707 7.5H11.5a.5.5 0 0 1 .5.5z"
            />
          </svg>
        </Button>
      )}
      <h1 className="text-lg whitespace-nowrap select-none">{title}</h1>
    </div>
  );
}
