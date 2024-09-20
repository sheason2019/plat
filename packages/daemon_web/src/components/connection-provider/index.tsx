import { PropsWithChildren, useEffect, useRef, useState } from "react";
import { x25519 } from "@noble/curves/ed25519";
import { sha3_256 } from "@noble/hashes/sha3";
import { base64url } from "@scure/base";
import ConnectionPending from "./pending";
import { ConnectionStatus } from "./typings";
import ConnectionFailed from "./failed";
import { useSearchParams } from "react-router-dom";

export default function ConnectionProvider({ children }: PropsWithChildren) {
  const [search] = useSearchParams();
  const [status, setStatus] = useState(ConnectionStatus.Pending);

  const [inputPassword, setInputPassword] = useState<
    ((pswd: string) => void) | null
  >(null);

  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    setStatus(ConnectionStatus.Pending);
    let sequence = 0;

    const ws = new WebSocket("/api/connect");

    const handleReceivePublicKey = async (base64UrlPublicKey: string) => {
      const pubKey = base64url.decode(base64UrlPublicKey);
      const privKey = x25519.utils.randomPrivateKey();
      const localPubKey = x25519.getPublicKey(privKey);

      const sharedSecret = x25519.getSharedSecret(privKey, pubKey);

      const passwordPromise = new Promise<string>((res) => {
        const password: string | null =
          search.get("password") ?? sessionStorage.getItem("password");

        if (password !== null) {
          return res(password);
        } else {
          setInputPassword(() => (pswd: string) => {
            sessionStorage.setItem("password", pswd);
            res(pswd);
            setInputPassword(null);
          });
        }
      });

      const password = await passwordPromise;
      const passwordBuf = new TextEncoder().encode(password);
      const passwordHash = sha3_256
        .create()
        .update(sharedSecret)
        .update(passwordBuf)
        .digest();

      ws.send(
        JSON.stringify({
          public_key: base64url.encode(localPubKey),
          password_hash: base64url.encode(passwordHash),
        })
      );
    };

    const handleReceiveResult = async (result: string) => {
      if (result === "OK") {
        return setStatus(ConnectionStatus.Success);
      }

      ws.close();
      sessionStorage.removeItem("password");
      return setStatus(ConnectionStatus.Failed);
    };

    ws.addEventListener("message", async (e) => {
      switch (sequence) {
        case 0:
          await handleReceivePublicKey(e.data);
          break;
        case 1:
          await handleReceiveResult(e.data);
          break;
        default:
          break;
      }

      sequence++;
    });

    wsRef.current = ws;

    return () => {
      ws.close();
      wsRef.current = null;
      setStatus(ConnectionStatus.Failed);
    };
  }, []);

  if (status === ConnectionStatus.Pending) {
    return <ConnectionPending inputPassword={inputPassword} />;
  }

  if (status === ConnectionStatus.Failed) {
    return <ConnectionFailed />;
  }

  return <>{children}</>;
}
