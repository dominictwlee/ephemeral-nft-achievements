import ms from "ms";
import * as anchor from "@project-serum/anchor";

export function secs(format: string) {
  return Math.floor(ms(format) / 1000);
}

export const SOL_DID_PUBLIC_KEY = new anchor.web3.PublicKey(
  "idDa4XeCjVwKcprVAo812coUQbovSZ4kDGJf2sPaBnM"
);
