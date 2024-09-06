import { PluginEntry, RegistedPlugin } from "../../../models/core";
import { atom, useRecoilState } from "recoil";

interface StageData {
  plugin: RegistedPlugin;
  entry: PluginEntry;
}

const stageState = atom<StageData | null>({
  key: "stage-data",
  default: null,
});

export default function useStage() {
  return useRecoilState(stageState);
}
