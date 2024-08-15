import { Entry, Plugin } from "../../../models/core";
import { atom, useRecoilState } from "recoil";

interface StageData {
  plugin: Plugin;
  entry: Entry;
}

const stageState = atom<StageData | null>({
  key: "stage-data",
  default: null,
});

export default function useStage() {
  return useRecoilState(stageState);
}
