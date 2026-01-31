import { WindowManager } from "../index";

// Demo: Managing multiple windows simultaneously
console.log("Creating WindowManager...");
const manager = new WindowManager();

// Start the event loop
manager.start();

// Create multiple windows
console.log("Creating multiple windows...");

const window1 = Number(manager.createWindowWithPosition(300, 300, "Window 1 - Red", 100, 100));
const window2 = Number(manager.createWindowWithPosition(300, 300, "Window 2 - Green", 450, 100));
const window3 = Number(manager.createWindowWithPosition(300, 300, "Window 3 - Blue", 800, 100));

console.log(`Created windows: ${window1}, ${window2}, ${window3}`);
console.log(`Total windows: ${manager.windowCount}`);

// Clear each window with different colors
manager.clear(window1, 255, 0, 0);    // Red
manager.clear(window2, 0, 255, 0);    // Green
manager.clear(window3, 0, 0, 255);    // Blue

// Draw identifying text patterns (pixel art style)
// Window 1: Draw "1"
for (let y = 100; y < 200; y++) {
  manager.setPixel(window1, 150, y, 255, 255, 255);
}
manager.setPixel(window1, 140, 110, 255, 255, 255);
manager.setPixel(window1, 160, 110, 255, 255, 255);

// Window 2: Draw "2"
for (let x = 100; x < 200; x++) {
  manager.setPixel(window2, x, 100, 255, 255, 255);
  manager.setPixel(window2, x, 150, 255, 255, 255);
  manager.setPixel(window2, x, 200, 255, 255, 255);
}
for (let y = 100; y < 150; y++) {
  manager.setPixel(window2, 200, y, 255, 255, 255);
}
for (let y = 150; y < 200; y++) {
  manager.setPixel(window2, 100, y, 255, 255, 255);
}

// Window 3: Draw "3"
for (let x = 100; x < 200; x++) {
  manager.setPixel(window3, x, 100, 255, 255, 255);
  manager.setPixel(window3, x, 150, 255, 255, 255);
  manager.setPixel(window3, x, 200, 255, 255, 255);
}
for (let y = 100; y < 200; y++) {
  manager.setPixel(window3, 200, y, 255, 255, 255);
}

// Present all windows
manager.present(window1);
manager.present(window2);
manager.present(window3);

console.log("All windows presented. Check your screen!");

// Demonstrate window operations
let tick = 0;
const interval = setInterval(() => {
  tick++;
  
  // Cycle through windows and add a moving pixel
  const activeWindow = [window1, window2, window3][tick % 3];
  const x = 50 + (tick * 10) % 200;
  const y = 250;
  
  // Draw a yellow dot that moves across each window in turn
  manager.setPixel(activeWindow, x, y, 255, 255, 0);
  manager.setPixel(activeWindow, x + 1, y, 255, 255, 0);
  manager.setPixel(activeWindow, x, y + 1, 255, 255, 0);
  manager.setPixel(activeWindow, x + 1, y + 1, 255, 255, 0);
  manager.present(activeWindow);
  
  console.log(`Tick ${tick}: Updated window ${activeWindow}, window count: ${manager.windowCount}`);
  
  if (tick >= 15) {
    clearInterval(interval);
    console.log("\nMultiple windows demo complete!");
    console.log("Windows are still open. Use windowExists() to check:");
    console.log(`  Window ${window1} exists: ${manager.windowExists(window1)}`);
    console.log(`  Window ${window2} exists: ${manager.windowExists(window2)}`);
    console.log(`  Window ${window3} exists: ${manager.windowExists(window3)}`);
  }
}, 500);

console.log("Main script completed - managing multiple windows!");
