import { createContext, useContext } from "react";
import { IDaemonContext } from "./typings";

export const daemonContext = createContext<IDaemonContext | null>(null);

export function useDaemonContext() {
  return useContext(daemonContext);
}
