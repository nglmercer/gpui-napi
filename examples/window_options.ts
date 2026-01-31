import { WindowManager } from "../index";

// Demo: Window options and management features
console.log("Creating WindowManager for window options demo...");
const manager = new WindowManager();

// Start the event loop
manager.start();

// Create windows with different options
console.log("\n=== Creating Windows with Different Options ===\n");

// 1. Basic window (default position)
const basicWindow = Number(manager.createWindow(300, 200, "1. Basic Window"));
manager.clear(basicWindow, 100, 100, 100);
drawLabel(basicWindow, 50, 100, "Default Position", 255, 255, 255);
manager.present(basicWindow);
console.log(`1. Basic window created (ID: ${basicWindow})`);

// 2. Window with specific position
const positionedWindow = Number(manager.createWindowWithPosition(
  300, 200, 
  "2. Positioned Window", 
  350, 100
));
manager.clear(positionedWindow, 0, 100, 150);
drawLabel(positionedWindow, 50, 100, "Position: 350,100", 255, 255, 255);
manager.present(positionedWindow);
console.log(`2. Positioned window created (ID: ${positionedWindow}) at (350, 100)`);

// 3. Always-on-top window
const topWindow = Number(manager.createWindowWithOptions(
  300, 200,
  "3. Always on Top",
  700, 100,
  true // alwaysOnTop
));
manager.clear(topWindow, 150, 0, 0);
drawLabel(topWindow, 50, 100, "Always On Top!", 255, 255, 255);
manager.present(topWindow);
console.log(`3. Always-on-top window created (ID: ${topWindow})`);

// 4. Window that we'll move dynamically
const movableWindow = Number(manager.createWindowWithPosition(
  300, 200,
  "4. Moving Window",
  100, 350
));
manager.clear(movableWindow, 0, 100, 0);
drawLabel(movableWindow, 50, 100, "Watch me move!", 255, 255, 255);
manager.present(movableWindow);
console.log(`4. Movable window created (ID: ${movableWindow}) at (100, 350)`);

// 5. Window with dynamic title
const titleWindow = Number(manager.createWindowWithPosition(
  300, 200,
  "5. Dynamic Title - Initial",
  450, 350
));
manager.clear(titleWindow, 100, 0, 100);
drawLabel(titleWindow, 50, 100, "Title changes...", 255, 255, 255);
manager.present(titleWindow);
console.log(`5. Dynamic title window created (ID: ${titleWindow})`);

console.log(`\nTotal windows: ${manager.windowCount}`);

// Demonstrate window operations
console.log("\n=== Demonstrating Window Operations ===\n");

let step = 0;
const operationsInterval = setInterval(() => {
  step++;
  
  switch (step) {
    case 1:
      // Move the movable window
      console.log("Step 1: Moving window to (400, 350)");
      manager.setPosition(movableWindow, 400, 350);
      manager.clear(movableWindow, 0, 150, 0);
      drawLabel(movableWindow, 50, 100, "Moved to 400,350", 255, 255, 255);
      manager.present(movableWindow);
      break;
      
    case 2:
      // Change title
      console.log("Step 2: Changing window title");
      manager.setTitle(titleWindow, "5. Dynamic Title - Updated!");
      manager.clear(titleWindow, 150, 0, 150);
      drawLabel(titleWindow, 50, 100, "Title Updated!", 255, 255, 255);
      manager.present(titleWindow);
      break;
      
    case 3:
      // Toggle always-on-top (create a new window with different setting)
      console.log("Step 3: Checking window existence");
      console.log(`  Window ${basicWindow} exists: ${manager.windowExists(basicWindow)}`);
      console.log(`  Window ${positionedWindow} exists: ${manager.windowExists(positionedWindow)}`);
      console.log(`  Window 99999 exists: ${manager.windowExists(99999)}`);
      break;
      
    case 4:
      // Change color of basic window
      console.log("Step 4: Changing basic window color");
      manager.clear(basicWindow, 200, 150, 50);
      drawLabel(basicWindow, 50, 100, "Color Changed!", 255, 255, 255);
      manager.present(basicWindow);
      break;
      
    case 5:
      // Move window again
      console.log("Step 5: Moving window again to (100, 100)");
      manager.setPosition(movableWindow, 100, 100);
      manager.clear(movableWindow, 50, 200, 50);
      drawLabel(movableWindow, 50, 100, "Back to 100,100", 255, 255, 255);
      manager.present(movableWindow);
      break;
      
    case 6:
      // Update title again
      console.log("Step 6: Final title update");
      manager.setTitle(titleWindow, "5. Dynamic Title - Final");
      manager.clear(titleWindow, 200, 100, 200);
      drawLabel(titleWindow, 50, 100, "All Done!", 255, 255, 255);
      manager.present(titleWindow);
      break;
      
    case 7:
      clearInterval(operationsInterval);
      console.log("\n=== Window Options Demo Complete ===");
      console.log(`\nFinal window count: ${manager.windowCount}`);
      console.log("\nAll windows are still open.");
      console.log("Try interacting with the 'Always on Top' window - it should stay above others.");
      break;
  }
}, 1500);

// Helper function to draw text-like patterns
function drawLabel(windowId: number, x: number, y: number, text: string, r: number, g: number, b: number) {
  // Draw a simple rectangle as a label background
  const padding = 10;
  const textWidth = text.length * 8;
  
  // Background
  for (let dy = -padding; dy < 20; dy++) {
    for (let dx = -padding; dx < textWidth + padding; dx++) {
      const px = x + dx;
      const py = y + dy;
      if (px >= 0 && px < 300 && py >= 0 && py < 200) {
        // Semi-transparent dark background
        manager.setPixel(windowId, px, py, 0, 0, 0);
      }
    }
  }
  
  // Draw some dots to represent text
  for (let i = 0; i < text.length; i++) {
    const charX = x + i * 8;
    // Draw a simple pattern for each "character"
    for (let dy = 0; dy < 8; dy++) {
      for (let dx = 0; dx < 6; dx++) {
        const px = charX + dx;
        const py = y + dy;
        if (px >= 0 && px < 300 && py >= 0 && py < 200) {
          // Simple pattern - vary by character position
          if ((dx + dy + i) % 2 === 0) {
            manager.setPixel(windowId, px, py, r, g, b);
          }
        }
      }
    }
  }
  
  // Draw border around label
  for (let dx = -padding; dx < textWidth + padding; dx++) {
    manager.setPixel(windowId, x + dx, y - padding, r, g, b);
    manager.setPixel(windowId, x + dx, y + 20, r, g, b);
  }
  for (let dy = -padding; dy <= 20; dy++) {
    manager.setPixel(windowId, x - padding, y + dy, r, g, b);
    manager.setPixel(windowId, x + textWidth + padding - 1, y + dy, r, g, b);
  }
}

console.log("\nWindow options demo running...");
