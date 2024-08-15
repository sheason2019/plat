import {
  Button,
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@nextui-org/react";
import { Link } from "@tanstack/react-router";

export default function AccountToggler() {
  return (
    <div className="flex-1 px-2">
      <Popover placement="top">
        <PopoverTrigger>
          <Button variant="flat" className="w-full h-full">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              fill="currentColor"
              viewBox="0 0 16 16"
            >
              <path
                fillRule="evenodd"
                d="M1 11.5a.5.5 0 0 0 .5.5h11.793l-3.147 3.146a.5.5 0 0 0 .708.708l4-4a.5.5 0 0 0 0-.708l-4-4a.5.5 0 0 0-.708.708L13.293 11H1.5a.5.5 0 0 0-.5.5zm14-7a.5.5 0 0 1-.5.5H2.707l3.147 3.146a.5.5 0 1 1-.708.708l-4-4a.5.5 0 0 1 0-.708l4-4a.5.5 0 1 1 .708.708L2.707 4H14.5a.5.5 0 0 1 .5.5z"
              />
            </svg>
            切换账号
          </Button>
        </PopoverTrigger>
        <PopoverContent>
          <div className="p-3">
            <p className="mb-3">即将移动到切换账号页面，确认要继续吗？</p>
            <div className="text-right">
              <Link to="/">
                <Button color="primary">确认</Button>
              </Link>
            </div>
          </div>
        </PopoverContent>
      </Popover>
    </div>
  );
}
