import { useParams } from "@tanstack/react-router";
import useProfile from "./use-profile";

export default function useIsolate() {
  const { pubkey } = useParams({ strict: false });

  const { data } = useProfile();
  return data.isolates.find((item) => item.public_key === pubkey);
}
