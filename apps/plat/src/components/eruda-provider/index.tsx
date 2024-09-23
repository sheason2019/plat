import eruda from "eruda";
import { PropsWithChildren, useEffect, useRef } from "react";
import { createPortal } from "react-dom";
import { useRecoilState } from "recoil";
import { erudaAtom } from "./atom";

export default function ErudaProvider({ children }: PropsWithChildren) {
  const [state] = useRecoilState(erudaAtom);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const anyEruda: any = eruda;
    if (state.show && !anyEruda._isInit) {
      try {
        eruda.init({
          container: containerRef.current!,
          useShadowDom: true,
          autoScale: true,
        });
      } catch (e) {
        console.warn("create eruda failed:", e);
      }
    }
    if (!state.show && anyEruda._isInit) {
      try {
        eruda.destroy();
      } catch (e) {
        console.warn("destroy eruda failed:", e);
      }
    }
  }, [state.show]);

  return (
    <>
      {children}
      {createPortal(
        <>{state.show && <div ref={containerRef} />}</>,
        document.body
      )}
    </>
  );
}
