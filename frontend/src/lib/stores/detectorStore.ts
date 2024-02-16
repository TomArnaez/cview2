import { writable } from "svelte/store";

export type CaptureProgressReport = {
    phase: string;
    message: string;
    step: number;
    totalSteps: number;
    estimatedTimeRemaining: string;
}

export interface ICapture {
    run(): AsyncIterableIterator<CaptureProgressReport>;
}

export class CaptureManager {
    async runCapture(captureTask: ICapture) {
        for await (const report of captureTask.run()) {
          console.log(`Phase: ${report.phase}, Message: ${report.message}, Step: ${report.step} of ${report.totalSteps}, Estimated Time Remaining: ${report.estimatedTimeRemaining}`);
        }
        console.log('Capture completed.');
    }
}

export class SequenceCapture implements ICapture {
    private numFrames: number;

    constructor(numFrames: number) {
        this.numFrames = numFrames;
    }

    async *run(): AsyncIterableIterator<CaptureProgressReport> {
        for (let step = 1; step <= this.numFrames; step++) {
          // Simulate capture delay
          await new Promise(resolve => setTimeout(resolve, 1000));
    
          const progressReport: CaptureProgressReport = {
            phase: "Capturing",
            message: `Captured image ${step} of ${this.numFrames}`,
            step,
            totalSteps: this.numFrames,
            estimatedTimeRemaining: `${this.numFrames - step} seconds remaining`
          };
    
          yield progressReport;
        }
    }
}
