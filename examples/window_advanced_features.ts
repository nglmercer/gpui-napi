import { WindowManager } from "../index";

// Demo: Advanced window features - reposition, always on top, and ignore input
console.log("Creating WindowManager for advanced features demo...");
const manager = new WindowManager();

// Start the event loop
manager.start();

// Create windows demonstrating different features
console.log("\n=== Creating Windows with Advanced Features ===\n");

// 1. Normal window (for comparison)
const normalWindow = Number(manager.createWindowWithPosition(300, 200, "1. Normal Window", 50, 50));
manager.clear(normalWindow, 50, 50, 50);
drawLabel(normalWindow, 50, 90, "Normal Window", 255, 255, 255);
drawLabel(normalWindow, 30, 120, "(Clickable)", 200, 200, 200);
manager.present(normalWindow);
console.log(`1. Normal window created (ID: ${normalWindow})`);

// 2. Always-on-top window
const topWindow = Number(manager.createWindowWithOptions(
  300, 200,
  "2. Always on Top",
  400, 50,
  true, // alwaysOnTop
  true,
  true
));
manager.clear(topWindow, 150, 0, 0);
drawLabel(topWindow, 50, 90, "Always On Top!", 255, 255, 255);
drawLabel(topWindow, 30, 120, "(Stays above others)", 200, 200, 200);
manager.present(topWindow);
console.log(`2. Always-on-top window created (ID: ${topWindow})`);

// 3. Window that will move around
const movingWindow = Number(manager.createWindowWithPosition(300, 200, "3. Moving Window", 750, 50));
manager.clear(movingWindow, 0, 100, 0);
drawLabel(movingWindow, 50, 90, "Watch Me Move!", 255, 255, 255);
drawLabel(movingWindow, 30, 120, "(Auto-repositioning)", 200, 200, 200);
manager.present(movingWindow);
console.log(`3. Moving window created (ID: ${movingWindow}) at (750, 50)`);

// 4. Click-through window (ignores input)
const clickThroughWindow = Number(manager.createWindowWithPosition(300, 200, "4. Click-Through", 50, 350));
manager.clear(clickThroughWindow, 0, 50, 150);
drawLabel(clickThroughWindow, 50, 90, "Click-Through!", 255, 255, 255);
drawLabel(clickThroughWindow, 20, 120, "(Clicks pass through)", 200, 200, 200);
manager.present(clickThroughWindow);
// Enable click-through mode - mouse events pass through to windows below
manager.setIgnoreInput(clickThroughWindow, true);
console.log(`4. Click-through window created (ID: ${clickThroughWindow}) - Input ignored`);

// 5. Toggle window (will toggle features)
const toggleWindow = Number(manager.createWindowWithPosition(300, 200, "5. Toggle Features", 400, 350));
manager.clear(toggleWindow, 100, 0, 100);
drawLabel(toggleWindow, 50, 90, "Feature Toggle", 255, 255, 255);
drawLabel(toggleWindow, 30, 120, "(See console)", 200, 200, 200);
manager.present(toggleWindow);
console.log(`5. Toggle window created (ID: ${toggleWindow})`);

console.log(`\nTotal windows: ${manager.windowCount}`);

// Demonstrate window operations
console.log("\n=== Demonstrating Advanced Features ===\n");

let step = 0;
const operationsInterval = setInterval(() => {
  step++;

  switch (step) {
    case 1:
      console.log("Step 1: Moving window to (750, 300)");
      manager.setPosition(movingWindow, 750, 300);
      manager.clear(movingWindow, 0, 150, 0);
      drawLabel(movingWindow, 50, 90, "Moved to 750,300", 255, 255, 255);
      drawLabel(movingWindow, 30, 120, "(New position!)", 200, 200, 200);
      manager.present(movingWindow);
      break;

    case 2:
      console.log("Step 2: Moving window again to (400, 500)");
      manager.setPosition(movingWindow, 400, 500);
      manager.clear(movingWindow, 0, 180, 0);
      drawLabel(movingWindow, 50, 90, "Moved to 400,500", 255, 255, 255);
      drawLabel(movingWindow, 30, 120, "(Another move!)", 200, 200, 200);
      manager.present(movingWindow);
      break;

    case 3:
      console.log("Step 3: Disabling click-through on window 4");
      manager.setIgnoreInput(clickThroughWindow, false);
      manager.clear(clickThroughWindow, 0, 100, 200);
      drawLabel(clickThroughWindow, 50, 90, "Now Clickable!", 255, 255, 255);
      drawLabel(clickThroughWindow, 30, 120, "(Input enabled)", 200, 200, 200);
      manager.present(clickThroughWindow);
      console.log("   Window 4 now accepts mouse/keyboard input");
      break;

    case 4:
      console.log("Step 4: Re-enabling click-through on window 4");
      manager.setIgnoreInput(clickThroughWindow, true);
      manager.clear(clickThroughWindow, 0, 50, 150);
      drawLabel(clickThroughWindow, 50, 90, "Click-Through!", 255, 255, 255);
      drawLabel(clickThroughWindow, 20, 120, "(Clicks pass through)", 200, 200, 200);
      manager.present(clickThroughWindow);
      console.log("   Window 4 now ignores input again");
      break;

    case 5:
      console.log("Step 5: Moving multiple windows");
      manager.setPosition(normalWindow, 100, 100);
      manager.setPosition(toggleWindow, 500, 400);
      manager.clear(normalWindow, 80, 80, 80);
      manager.clear(toggleWindow, 150, 0, 150);
      drawLabel(normalWindow, 50, 90, "Also Moved!", 255, 255, 255);
      drawLabel(toggleWindow, 50, 90, "Moved Too!", 255, 255, 255);
      manager.present(normalWindow);
      manager.present(toggleWindow);
      console.log("   Normal window: (50,50) -> (100,100)");
      console.log("   Toggle window: (400,350) -> (500,400)");
      break;

    case 6:
      console.log("Step 6: Final positions");
      manager.setPosition(movingWindow, 200, 200);
      manager.clear(movingWindow, 50, 200, 50);
      drawLabel(movingWindow, 50, 90, "Final: 200,200", 255, 255, 255);
      manager.present(movingWindow);
      break;

    case 7:
      clearInterval(operationsInterval);
      console.log("\n=== Advanced Features Demo Complete ===");
      console.log(`\nFinal window count: ${manager.windowCount}`);
      console.log("\nFeature Summary:");
      console.log("  - Window 1 (Normal): Standard behavior");
      console.log("  - Window 2 (Always on Top): Stays above other windows");
      console.log("  - Window 3 (Moving): Demonstrated setPosition()");
      console.log("  - Window 4 (Click-Through): Ignores mouse/keyboard input");
      console.log("  - Window 5 (Toggle): Demonstrated feature toggling");
      console.log("\nTry clicking on the click-through window - clicks pass through!");
      console.log("Try clicking on the always-on-top window - it stays on top!");
      break;
  }
}, 2000);

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

console.log("\nAdvanced features demo running...");
