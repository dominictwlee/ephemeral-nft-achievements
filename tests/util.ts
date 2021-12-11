import ms from "ms";
import * as BufferLayout from "buffer-layout";
import * as borsh from "@project-serum/borsh";

export function secs(format: string) {
  return Math.floor(ms(format) / 1000);
}

export const ProfileLayout = (alias: string, uri: string) =>
  borsh.struct([
    borsh.publicKey("owner"),
    borsh.u8("bump"),
    borsh.str("alias"),
    borsh.vec(borsh.publicKey(), "delegates"),
    borsh.str("details_uri"),
  ]);

export function publicKeyLayout(property?: string) {
  return BufferLayout.blob(32, property);
}

export function uint64Layout(property?: string) {
  return BufferLayout.blob(8, property);
}

export function rustStringLayout(property = "string") {
  return BufferLayout.struct(
    [
      BufferLayout.u32("length"),
      BufferLayout.u32("lengthPadding"),
      BufferLayout.blob(BufferLayout.offset(BufferLayout.u32(), -8), "chars"),
    ],
    property
  );
}

export function vecLayout(property = "vec") {
  return BufferLayout.struct(
    [
      BufferLayout.u32("length"),
      BufferLayout.u32("lengthPadding"),
      BufferLayout.blob(BufferLayout.offset(BufferLayout.u32(), -8), "chars"),
    ],
    property
  );
}
