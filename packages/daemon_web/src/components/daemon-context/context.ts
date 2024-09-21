import { createContext, useContext } from "react";

export interface IDaemonContext {
  fromOrigin: string;
  password: string;
}

export const daemonContext = createContext<IDaemonContext | null>(null);

export function useDaemonContext() {
  return useContext(daemonContext);
}
