import { useMemo } from "react";

export default function useOrigin(): string {
  return useMemo(() => {
    if (window.__POWERED_BY_WUJIE__) {
      return window.__WUJIE.url;
    } else {
      return window.location.origin;
    }
  }, []);
}
