import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";
import sharp from "sharp";

const TARGET_PIXELS = 2550;
function targetDims(w, h) {
  if (w * h <= TARGET_PIXELS) return [w, h];
  const ratio = w / h;
  return [
    Math.max(1, Math.round(Math.sqrt(TARGET_PIXELS * ratio))),
    Math.max(1, Math.round(Math.sqrt(TARGET_PIXELS / ratio))),
  ];
}

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = join(__dirname, "..");
const imgPath = join(root, "public/default.webp");

const meta = await sharp(imgPath).metadata();
const [w, h] = targetDims(meta.width, meta.height);
const rgba = await sharp(imgPath)
  .resize(w, h, { kernel: "nearest" })
  .ensureAlpha()
  .raw()
  .toBuffer();

const require = createRequire(import.meta.url);
const wasm = require(join(root, "../thaimeleon_lib/pkg-node/thaimeleon_lib.js"));

const FORCE_LIGHT = 1;
const FORCE_DARK = 0;
const light = wasm.generate_scheme(rgba, w, h, FORCE_LIGHT, undefined);
const dark = wasm.generate_scheme(rgba, w, h, FORCE_DARK, undefined);

const out = join(root, "src/defaultSchemes.json");
const payload = {
  width: w,
  height: h,
  rgba: Buffer.from(rgba).toString("base64"),
  light,
  dark,
};
writeFileSync(out, JSON.stringify(payload, null, 2) + "\n");
console.log(`wrote ${out} (${w}x${h})`);
