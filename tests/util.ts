import ms from "ms";

export function secs(format: string) {
  return Math.floor(ms(format) / 1000);
}
