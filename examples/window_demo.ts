import { WindowManager } from "../index";

// Demo: Non-blocking WindowManager with pixel manipulation
console.log("Creating WindowManager...");
const manager = new WindowManager();

// Start the event loop
manager.start();

// Create a window
console.log("Creating window...");
const windowId = manager.createWindow(400, 400, "Non-blocking Window Demo");
console.log(`Window created with ID: ${windowId}`);

// Convert bigint to number for the API
const id = Number(windowId);

// Clear with black
manager.clearBlack(id);

// Draw a red diagonal line
for (let i = 0; i < 400; i++) {
  manager.setPixel(id, i, i, 255, 0, 0);
}

// Draw a green horizontal line
for (let x = 50; x < 350; x++) {
  manager.setPixel(id, x, 200, 0, 255, 0);
}

// Draw a blue vertical line
for (let y = 50; y < 350; y++) {
  manager.setPixel(id, 200, y, 0, 0, 255);
}

// Present the rendered pixels
manager.present(id);
console.log("Window presented. You can close it anytime.");

// Demonstrate non-blocking behavior
let counter = 0;
const interval = setInterval(() => {
  counter++;
  console.log(`Non-blocking tick: ${counter}`);
  
  // Animate something - move a pixel
  const x = 100 + (counter % 200);
  manager.setPixel(id, x, 100, 255, 255, 0);
  manager.present(id);
  if (counter >= 10) {
    clearInterval(interval);
    console.log("Demo complete. Window is still open.");
  }
}, 1000);

console.log("Main script completed - but the window stays open!");
