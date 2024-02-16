<script>
  import "./styles.css";
  import {CaptureManager, SequenceCapture} from "./core/capture";

  let captureManager = new CaptureManager();
  let captureTask = new SequenceCapture(10); // Example initialization with 10 steps

// Function to start the capture process
async function startCapture() {
  await captureManager.runCapture(captureTask);
  console.log('Capture process completed');
}

</script>

<div class="flex h-screen">
  <!-- Sidebar -->
  <div class="w-64 bg-gray-800 text-white">
    <!-- Sidebar content here -->
    <div class="p-5">Sidebar</div>
  </div>

  <!-- Main Content -->
  <div class="flex-1 bg-gray-100">    
    <div class="p-5">Main Content</div>
    <button on:click="{startCapture}">Start Capture</button>
  </div>
</div>