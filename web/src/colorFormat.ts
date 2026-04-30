export type Format = "rgb" | "rgba" | "oklch";

function hexToRgb(hex: string): [number, number, number] {
  return [
    parseInt(hex.slice(1, 3), 16),
    parseInt(hex.slice(3, 5), 16),
    parseInt(hex.slice(5, 7), 16),
  ];
}

function srgbToLinear(c: number): number {
  const x = c / 255;
  return x <= 0.04045 ? x / 12.92 : Math.pow((x + 0.055) / 1.055, 2.4);
}

function rgbToOklab(r: number, g: number, b: number): [number, number, number] {
  const lr = srgbToLinear(r);
  const lg = srgbToLinear(g);
  const lb = srgbToLinear(b);
  const l = 0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb;
  const m = 0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb;
  const s = 0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb;
  const l_ = Math.cbrt(l);
  const m_ = Math.cbrt(m);
  const s_ = Math.cbrt(s);
  return [
    0.2104542553 * l_ + 0.793617785 * m_ - 0.0040720468 * s_,
    1.9779984951 * l_ - 2.428592205 * m_ + 0.4505937099 * s_,
    0.0259040371 * l_ + 0.7827717662 * m_ - 0.808675766 * s_,
  ];
}

function hexToOklch(hex: string): string {
  const [r, g, b] = hexToRgb(hex);
  const [L, a, bb] = rgbToOklab(r, g, b);
  const C = Math.sqrt(a * a + bb * bb);
  let H = (Math.atan2(bb, a) * 180) / Math.PI;
  if (H < 0) H += 360;
  return `oklch(${(L * 100).toFixed(1)}% ${C.toFixed(3)} ${H.toFixed(1)})`;
}

function hexToRgba(hex: string): string {
  const [r, g, b] = hexToRgb(hex);
  return `rgba(${r}, ${g}, ${b}, 1)`;
}

export function format(hex: string, fmt: Format): string {
  if (fmt === "rgb") return hex;
  if (fmt === "rgba") return hexToRgba(hex);
  return hexToOklch(hex);
}

const HEX_RE = /^#[0-9a-fA-F]{6}$/;

export function formatScheme<T>(value: T, fmt: Format): T {
  if (typeof value === "string") {
    return (HEX_RE.test(value) ? format(value, fmt) : value) as T;
  }
  if (Array.isArray(value)) {
    return value.map((v) => formatScheme(v, fmt)) as T;
  }
  if (value && typeof value === "object") {
    const out: Record<string, unknown> = {};
    for (const [k, v] of Object.entries(value)) {
      out[k] = formatScheme(v, fmt);
    }
    return out as T;
  }
  return value;
}
