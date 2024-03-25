export type View = {
    id: string,
    width: number,
    height: number,
    stack_size: number
    buffer: Uint8ClampedArray,
    settings: ImageViewSettings,
}

type WithBuffer = {
    buffer: Uint8ClampedArray,
    roi: Rect | null
};

export type ViewWithBuffer = View & WithBuffer;

export type SaturatedPixelSettings = {
    saturatedLimit: number,
    saturatedColour: number[]
}

export type ImageViewSettings = {
    histogramEquilisation: boolean,
    invertColours: boolean,
    saturationPixelSettings: SaturatedPixelSettings | undefined
}

export type DrawTool = "Rectangle" | "Line" | undefined;

export type Point = {
    x: number,
    y: number
}

export type Rect = {
    width: number,
    height: number,
    pos: Point
}

export type Line = {
    start: Point,
    end: Point
}