import { useCallback, useRef } from "react";

type Props = {
  onImage: (
    p: { rgba: Uint8Array; width: number; height: number; url: string },
  ) => void;
  onError: (msg: string) => void;
  preview?: string;
};

const TARGET_PIXELS = 2550;

function targetDims(w: number, h: number): [number, number] {
  if (w * h <= TARGET_PIXELS) return [w, h];
  const ratio = w / h;
  const newW = Math.max(1, Math.round(Math.sqrt(TARGET_PIXELS * ratio)));
  const newH = Math.max(1, Math.round(Math.sqrt(TARGET_PIXELS / ratio)));
  return [newW, newH];
}

export async function decodeFromUrl(url: string) {
  const img = new Image();
  img.src = url;
  await img.decode();
  const [w, h] = targetDims(img.naturalWidth, img.naturalHeight);
  const canvas = document.createElement("canvas");
  canvas.width = w;
  canvas.height = h;
  const ctx = canvas.getContext("2d")!;
  ctx.imageSmoothingEnabled = false;
  ctx.drawImage(img, 0, 0, w, h);
  const data = ctx.getImageData(0, 0, w, h);
  return {
    rgba: new Uint8Array(data.data.buffer),
    width: w,
    height: h,
    url,
  };
}

async function decode(file: File) {
  return decodeFromUrl(URL.createObjectURL(file));
}

export default function ImagePicker({ onImage, onError, preview }: Props) {
  const inputRef = useRef<HTMLInputElement>(null);

  const handle = useCallback(
    async (file?: File | null) => {
      if (!file) return;
      if (file.size === 0) {
        onError("file is empty");
        return;
      }
      if (file.type && !file.type.startsWith("image/")) {
        onError(
          `not an image file (got ${file.type || "unknown"}). try png, jpg, webp, etc.`,
        );
        return;
      }
      try {
        onImage(await decode(file));
      } catch {
        onError(
          "could not decode image. file may be corrupted or in an unsupported format.",
        );
      }
    },
    [onImage, onError],
  );

  const onDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      handle(e.dataTransfer.files[0]);
    },
    [handle],
  );

  const open = () => inputRef.current?.click();
  return (
    <section
      className={"picker" + (preview ? " has-image" : "")}
      onDragOver={(e) => e.preventDefault()}
      onDrop={onDrop}
    >
      <input
        ref={inputRef}
        type="file"
        accept="image/*"
        hidden
        onChange={(e) => handle(e.target.files?.[0])}
      />
      {preview ? (
        <>
          <img src={preview} alt="" />
          <button className="new-image" onClick={open}>new image</button>
        </>
      ) : (
        <button className="dropzone" onClick={open}>
          drop image or click
        </button>
      )}
    </section>
  );
}
