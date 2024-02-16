import { SLImage } from "../stores/imageStore";
import type { PixelDataArray } from "../stores/imageStore";
import { ok, err, Result } from "neverthrow";

export function convertImageToRGBACanvas<T extends PixelDataArray>(image: SLImage<T>) : Result<HTMLCanvasElement, Error> {
    const canvas: HTMLCanvasElement = document.createElement('canvas');
    canvas.width = image.width;
    canvas.height = image.height;

    const ctx = canvas.getContext('2d');
    if (!ctx) {
        return err(new Error("Unable to get canvas context"));
    }

    const imageData = ctx.createImageData(image.width, image.height);

    // TODO: Handle different data types
    const data = new Uint16Array(image.data);

    for (let i = 0; i < data.length; i++) {
        const value = data[i]; // 16-bit value
        const scaledValue = (value >> 8) & 0xFF; // Scale down to 8-bit

        // Set RGBA values for grayscale; R=G=B and A=255 (fully opaque)
        // imageData.data[i * 4] = scaledValue;     // R
        // imageData.data[i * 4 + 1] = scaledValue; // G
        // imageData.data[i * 4 + 2] = scaledValue; // B
        // imageData.data[i * 4 + 3] = 255;         // A

        imageData.data[i * 4] = 0;     // R
        imageData.data[i * 4 + 1] = 255; // G
        imageData.data[i * 4 + 2] = 0; // B
        imageData.data[i * 4 + 3] = 255;         // A
    }

    ctx.putImageData(imageData, 0, 0);

    return ok(canvas);
}