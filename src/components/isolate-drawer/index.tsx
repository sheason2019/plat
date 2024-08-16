import { Card, CardBody } from "@nextui-org/react";
import { AnimatePresence, motion } from "framer-motion";
import useIsolate from "../../hooks/core/use-isolate";
import EntryButton from "./buttons/entry-button";
import { Fragment } from "react/jsx-runtime";
import useIsolateDrawer from "./hooks/use-isolate-drawer";
import DrawerToggler from "./buttons/drawer-toggler";
import SettingButton from "./buttons/setting-button";
import AccountToggler from "./buttons/account-toggler";

export default function IsolateDrawer() {
  const isolate = useIsolate();
  const { isOpen, onClose } = useIsolateDrawer();

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
            <div className="flex flex-col flex-1">
              <div className="flex items-stretch flex-1">
                <div className="flex-1 px-2"></div>
                <div className="w-16 flex flex-col mr-2">
                  {isolate?.plugins.map((plugin) => (
                    <Fragment key={plugin.config.name}>
                      {plugin.config.entries.map((entry) => (
                        <EntryButton
                          key={entry.label}
                          plugin={plugin}
                          entry={entry}
                        />
                      ))}
                    </Fragment>
                  ))}
                </div>
              </div>
              <div className="shrink-0 mb-1 px-2 flex items-stretch">
                <SettingButton />
                <AccountToggler />
                <DrawerToggler />
              </div>
            </div>
          </CardBody>
        </Card>
      </motion.div>
    </>
  );
}
