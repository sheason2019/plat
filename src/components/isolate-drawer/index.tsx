import { Button, Card, CardBody, useDisclosure } from "@nextui-org/react";
import { Link } from "@tanstack/react-router";
import { AnimatePresence, motion } from "framer-motion";
import useIsolate from "../../hooks/core/use-isolate";
import EntryButton from "./entry-button";
import { Fragment } from "react/jsx-runtime";

export default function IsolateDrawer() {
  const isolate = useIsolate();
  const { isOpen, onOpenChange, onClose } = useDisclosure();

  return (
    <>
      <div className="w-20" />
      <AnimatePresence>
        {isOpen && (
          <motion.div
            key="overlay"
            onClick={onClose}
            className="fixed inset-0 bg-slate-500"
            initial={{ opacity: 0 }}
            animate={{ opacity: 0.35 }}
            exit={{ opacity: 0 }}
          />
        )}
      </AnimatePresence>
      <motion.div
        className="absolute left-0 top-0 bottom-0 max-w-sm w-full"
        initial={{ translateX: "calc(0px - 100%)" }}
        animate={{
          translateX: isOpen ? "0" : "calc(96px - 100%)",
        }}
        transition={{
          type: "spring",
          duration: 0.35,
        }}
      >
        <Card className="bg-blue-50 rounded-l-none h-full mr-4">
          <CardBody className="py-2 px-0">
            <div className="flex items-stretch flex-1">
              <div className="flex-1 px-2">
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
              <div className="flex flex-col h-full items-center w-16 mx-2 shrink-0">
                <div className="flex-1 flex flex-col items-center">
                  {isolate?.plugins.map((plugin) => (
                    <Fragment key={plugin.name}>
                      {plugin.entries.map((entry) => (
                        <EntryButton
                          key={entry.name}
                          plugin={plugin}
                          entry={entry}
                        />
                      ))}
                    </Fragment>
                  ))}
                </div>
                <div className="mb-1">
                  <Button
                    isIconOnly
                    size="lg"
                    onClick={onOpenChange}
                    color="primary"
                    className="delay-100"
                    as={motion.button}
                    animate={{
                      rotate: isOpen ? 180 : 0,
                    }}
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width={20}
                      height={20}
                      fill="currentColor"
                      viewBox="0 0 16 16"
                    >
                      <path
                        fillRule="evenodd"
                        d="M4.146 3.646a.5.5 0 0 0 0 .708L7.793 8l-3.647 3.646a.5.5 0 0 0 .708.708l4-4a.5.5 0 0 0 0-.708l-4-4a.5.5 0 0 0-.708 0zM11.5 1a.5.5 0 0 1 .5.5v13a.5.5 0 0 1-1 0v-13a.5.5 0 0 1 .5-.5z"
                      />
                    </svg>
                  </Button>
                </div>
              </div>
            </div>
          </CardBody>
        </Card>
      </motion.div>
    </>
  );
}
