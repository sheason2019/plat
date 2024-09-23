import { IConnectionContext } from "./typings";
import { atom } from "recoil";

export const connectionState = atom<IConnectionContext | null>({
  key: "connectionState",
  default: null,
});
