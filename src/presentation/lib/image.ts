/** Browser-side image processing for spray creation. */

export interface SprayImageData {
  name: string;
  width: number;
  height: number;
  rgbaBase64: string;
}

const SPRAY_SIZE = 512;

/**
 * Decode an image file, fit it (preserving aspect, transparent padding) into a
 * 512x512 canvas, and return its RGBA pixels base64-encoded plus a sanitized
 * name derived from the file name.
 */
export async function fileToSprayImage(file: File): Promise<SprayImageData> {
  const url = URL.createObjectURL(file);
  try {
    const img = await loadImage(url);
    const canvas = document.createElement("canvas");
    canvas.width = SPRAY_SIZE;
    canvas.height = SPRAY_SIZE;
    const ctx = canvas.getContext("2d", { willReadFrequently: true });
    if (!ctx) throw new Error("Canvas 2D context unavailable");

    ctx.clearRect(0, 0, SPRAY_SIZE, SPRAY_SIZE); // keep transparency

    const scale = Math.min(SPRAY_SIZE / img.width, SPRAY_SIZE / img.height);
    const w = img.width * scale;
    const h = img.height * scale;
    ctx.drawImage(img, (SPRAY_SIZE - w) / 2, (SPRAY_SIZE - h) / 2, w, h);

    const { data } = ctx.getImageData(0, 0, SPRAY_SIZE, SPRAY_SIZE);
    return {
      name: sanitizeName(file.name),
      width: SPRAY_SIZE,
      height: SPRAY_SIZE,
      rgbaBase64: bytesToBase64(new Uint8Array(data.buffer)),
    };
  } finally {
    URL.revokeObjectURL(url);
  }
}

function loadImage(src: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = () => reject(new Error("Could not decode the image"));
    img.src = src;
  });
}

/** Derive a safe spray name (letters/digits/_/-) from a file name. */
export function sanitizeName(fileName: string): string {
  const stem = fileName.replace(/\.[^.]+$/, "");
  const cleaned = stem.replace(/[^A-Za-z0-9_-]+/g, "_").replace(/^_+|_+$/g, "");
  return cleaned || "spray";
}

function bytesToBase64(bytes: Uint8Array): string {
  let binary = "";
  const chunk = 0x8000;
  for (let i = 0; i < bytes.length; i += chunk) {
    binary += String.fromCharCode(...bytes.subarray(i, i + chunk));
  }
  return btoa(binary);
}
