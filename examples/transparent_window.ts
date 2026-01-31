import index from "../index.js";
const { WindowManager } = index;

// Demo: Transparent windows and RGBA rendering
console.log("Creating WindowManager for transparent window demo...");
const manager = new WindowManager();

// Start the event loop
manager.start();
const glassWindow = Number(manager.createWindowWithOptions(
  300, 200,
  "5. Glass Effect",
  400, 350,
  false, // alwaysOnTop
  false,  // transparent
  true  // decorations
));
// Create a frosted glass effect with semi-transparent white
drawGlassEffect(glassWindow);
drawLabel(glassWindow, 50, 90, "Glass Effect!", 50, 50, 50);
manager.present(glassWindow);
console.log(`5. Glass effect window created (ID: ${glassWindow})`);

console.log(`\nTotal windows: ${manager.windowCount}`);

// Demonstrate transparency features
console.log("\n=== Demonstrating Transparency Features ===\n");

let step = 0;
const operationsInterval = setInterval(() => {
  step++;

  switch (step) {

    case 5:
      clearInterval(operationsInterval);
      console.log(`\nFinal window count: ${manager.windowCount}`);
      break;
  }
}, 2000);

// Helper function to draw text-like patterns
function drawLabel(windowId: number, x: number, y: number, text: string, r: number, g: number, b: number) {
  const padding = 10;
  const textWidth = text.length * 8;

  // Background
  for (let dy = -padding; dy < 20; dy++) {
    for (let dx = -padding; dx < textWidth + padding; dx++) {
      const px = x + dx;
      const py = y + dy;
      if (px >= 0 && px < 300 && py >= 0 && py < 200) {
        manager.setPixel(windowId, px, py, 0, 0, 0);
      }
    }
  }

  // Draw some dots to represent text
  for (let i = 0; i < text.length; i++) {
    const charX = x + i * 8;
    for (let dy = 0; dy < 8; dy++) {
      for (let dx = 0; dx < 6; dx++) {
        const px = charX + dx;
        const py = y + dy;
        if (px >= 0 && px < 300 && py >= 0 && py < 200) {
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

// Draw a glass effect (frosted glass look)
function drawGlassEffect(windowId: number) {
  // Base semi-transparent white
  for (let y = 0; y < 200; y++) {
    for (let x = 0; x < 300; x++) {
      const alpha = 180; // Semi-transparent
      manager.setPixelRgba(windowId, x, y, 255, 255, 255, alpha);
    }
  }

  // Add some "noise" for frosted effect
  for (let i = 0; i < 500; i++) {
    const x = Math.floor(Math.random() * 300);
    const y = Math.floor(Math.random() * 200);
    const shade = Math.floor(Math.random() * 50);
    manager.setPixelRgba(windowId, x, y, 255 - shade, 255 - shade, 255 - shade, 200);
  }
}
