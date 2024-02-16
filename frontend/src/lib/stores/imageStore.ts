import { writable } from 'svelte/store';
import { Result, err, ok } from 'neverthrow';

type PixelDataArray = Uint16Array | Float32Array;

type Rectangle = {
    type: "rectangle",
    x: number,
    y: number,
    width: number,
    height: number
}

type Line = {
    type: "line",
    centre: { x: number, y: number },
    radius: number
}

type ROI = Rectangle | Line;

class Image<T extends PixelDataArray> {
    #data: T;
    #width: number;
    #height: number;

    constructor(data: T, width: number, height: number) {
        this.#data = data;
        this.#width = width;
        this.#height = height;
    }

    get width(): number {
        return this.#width;
    }

    get height(): number {
        return this.#height;
    }

    getPixelValue(x: number, y: number): Result<number, Error> {
        if (x < 0 || x >= this.#width || y < 0 || y >= this.#height) {
            return err(new Error("Position out of bounds"));
        }
        return ok(this.#data[y * this.#width + x]);
    }
}
const images = writable<Image<PixelDataArray>[]>([]);
