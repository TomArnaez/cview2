import { invoke } from "../backend/ipc";

export type Image = {
    id: string
    width: number
    height: number
}

export interface IView {
    getImageData(): ImageData;
    getPixelValue(x: number, y: number): void;
    histogramEquilization(): void;
}
  
export async function createStaticImageView(imageId: String): Promise<IView> {
    const imageData = createRedImageData(300, 300);

    window.addEventListener('sharedbufferreceived', function listener(event) {
        if (event.additionalData && event.additionalData.id === "test") {
          console.log('Condition met, removing this listener.');
          window.removeEventListener('customEvent', listener);
        } else {
          console.log('Condition not met, listener remains.');
        }
      });

    await invoke<any>("create_image_view", { imageId });
  
    function getImageData(): ImageData {
        return imageData;
    }
  
    function getPixelValue(x: number, y: number): void {
    }
  
    function histogramEquilization(): void {
    }
  
    return { getImageData, getPixelValue, histogramEquilization };
}
  
function createRedImageData(width: number, height: number): ImageData {
    const imageData = new ImageData(width, height);
    for (let i = 0; i < imageData.data.length; i += 4) {
        imageData.data[i] = 255; // Red
        imageData.data[i + 1] = 0; // Green
        imageData.data[i + 2] = 0; // Blue
        imageData.data[i + 3] = 255; // Alpha
    }
    return imageData;
}

export type ImageViewEvent = {
}

