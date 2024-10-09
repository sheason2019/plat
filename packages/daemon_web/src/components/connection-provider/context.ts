import { IConnectionContext } from "./typings";
import { atom } from "recoil";

export const connectionState = atom<IConnectionContext>({
  key: "connectionState",
  default: {},
});
