import { useParams } from "@tanstack/react-router";
import useProfile from "./use-profile";

export default function useDaemon() {
  const { pubkey } = useParams({ strict: false });

  const { data } = useProfile();
  return data.daemons.find((item) => item.public_key === pubkey);
}
