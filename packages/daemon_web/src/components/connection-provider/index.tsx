import { PropsWithChildren, useEffect, useRef, useState } from "react";
import { x25519 } from "@noble/curves/ed25519";
import { sha3_256 } from "@noble/hashes/sha3";
import { base64url } from "@scure/base";

export default function ConnectionProvider({ children }: PropsWithChildren) {
  const [isOk, setIsOk] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    setIsOk(false);
    let sequence = 0;

    const ws = new WebSocket("/api/connect");

    const handleReceivePublicKey = async (base64UrlPublicKey: string) => {
      const pubKey = base64url.decode(base64UrlPublicKey);
      const privKey = x25519.utils.randomPrivateKey();
      const localPubKey = x25519.getPublicKey(privKey);

      const sharedSecret = x25519.getSharedSecret(privKey, pubKey);

      const password = "TODO";
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
        return setIsOk(true);
      }

      ws.close();
      setIsOk(false);
    };

    ws.addEventListener("message", async (e) => {
      console.log("message", e);
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
      setIsOk(false);
    };
  }, []);

  if (!isOk) {
    return null;
  }

  return <>{children}</>;
}
