import { WindowManager } from "../index";

// Demo: Transparent windows and RGBA rendering
console.log("Creating WindowManager for transparent window demo...");
const manager = new WindowManager();

// Start the event loop
manager.start();

console.log("\n=== Creating Transparent and Frameless Windows ===\n");

// 1. Normal window with frame (for comparison)
const normalWindow = Number(manager.createWindowWithOptions(
  300, 200,
  "1. Normal Window",
  50, 50,
  false, // alwaysOnTop
  false, // transparent
  true   // decorations (frame)
));
manager.clear(normalWindow, 100, 100, 100);
drawLabel(normalWindow, 50, 90, "Normal + Frame", 255, 255, 255);
manager.present(normalWindow);
console.log(`1. Normal window with frame created (ID: ${normalWindow})`);

// 2. Transparent window with frame
const transparentWindow = Number(manager.createWindowWithOptions(
  300, 200,
  "2. Transparent + Frame",
  400, 50,
  false, // alwaysOnTop
  true,  // transparent
  true   // decorations (frame)
));
// Clear with semi-transparent background
manager.clear(transparentWindow, 0, 0, 0); // This will be black but we can draw transparent pixels
drawLabel(transparentWindow, 50, 90, "Transparent!", 255, 100, 100);
// Draw a semi-transparent overlay
for (let y = 50; y < 150; y++) {
  for (let x = 50; x < 250; x++) {
    manager.setPixelRgba(transparentWindow, x, y, 0, 100, 200, 128); // 50% transparent blue
  }
}
manager.present(transparentWindow);
console.log(`2. Transparent window with frame created (ID: ${transparentWindow})`);

// 3. Frameless window (no decorations)
const framelessWindow = Number(manager.createWindowWithOptions(
  300, 200,
  "3. Frameless Window",
  750, 50,
  false, // alwaysOnTop
  false, // transparent
  false  // no decorations (frameless)
));
manager.clear(framelessWindow, 50, 100, 50);
drawLabel(framelessWindow, 50, 90, "Frameless!", 255, 255, 255);
manager.present(framelessWindow);
console.log(`3. Frameless window created (ID: ${framelessWindow})`);

// 4. Fully transparent frameless window (overlay style)
const overlayWindow = Number(manager.createWindowWithOptions(
  300, 200,
  "4. Overlay Window",
  50, 350,
  true,  // alwaysOnTop
  true,  // transparent
  false  // no decorations
));
// Draw a gradient with transparency
for (let y = 0; y < 200; y++) {
  for (let x = 0; x < 300; x++) {
    const alpha = Math.floor((1 - y / 200) * 200); // Fade from opaque to transparent
    const r = Math.floor((x / 300) * 255);
    const g = Math.floor((y / 200) * 255);
    const b = 100;
    manager.setPixelRgba(overlayWindow, x, y, r, g, b, alpha);
  }
}
drawLabel(overlayWindow, 50, 90, "RGBA Overlay!", 255, 255, 255);
manager.present(overlayWindow);
console.log(`4. Transparent frameless overlay created (ID: ${overlayWindow})`);

// 5. Glass effect window
const glassWindow = Number(manager.createWindowWithOptions(
  300, 200,
  "5. Glass Effect",
  400, 350,
  false, // alwaysOnTop
  true,  // transparent
  true   // decorations
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
    case 1:
      console.log("Step 1: Adding semi-transparent shapes to overlay");
      // Draw a pulsing circle effect
      drawCircle(overlayWindow, 150, 100, 50, 255, 255, 0, 180);
      manager.present(overlayWindow);
      break;

    case 2:
      console.log("Step 2: Updating transparent window with new content");
      manager.clear(transparentWindow, 0, 0, 0);
      // Draw a checkerboard pattern with transparency
      for (let y = 0; y < 200; y += 20) {
        for (let x = 0; x < 300; x += 20) {
          const alpha = ((x / 20 + y / 20) % 2 === 0) ? 200 : 50;
          for (let dy = 0; dy < 20; dy++) {
            for (let dx = 0; dx < 20; dx++) {
              manager.setPixelRgba(transparentWindow, x + dx, y + dy, 255, 100, 150, alpha);
            }
          }
        }
      }
      drawLabel(transparentWindow, 50, 90, "Checkerboard!", 255, 255, 255);
      manager.present(transparentWindow);
      break;

    case 3:
      console.log("Step 3: Adding shadow effect to frameless window");
      drawShadowEffect(framelessWindow);
      drawLabel(framelessWindow, 50, 90, "With Shadow!", 255, 255, 255);
      manager.present(framelessWindow);
      break;

    case 4:
      console.log("Step 4: Final transparency demo");
      // Make the overlay window fully transparent except for text
      for (let y = 0; y < 200; y++) {
        for (let x = 0; x < 300; x++) {
          manager.setPixelRgba(overlayWindow, x, y, 0, 0, 0, 0); // Fully transparent
        }
      }
      drawLabel(overlayWindow, 50, 90, "Almost Invisible!", 255, 255, 255);
      manager.present(overlayWindow);
      break;

    case 5:
      clearInterval(operationsInterval);
      console.log("\n=== Transparency Demo Complete ===");
      console.log(`\nFinal window count: ${manager.windowCount}`);
      console.log("\nFeature Summary:");
      console.log("  - Window 1: Normal window with frame (baseline)");
      console.log("  - Window 2: Transparent window with frame");
      console.log("  - Window 3: Frameless window (no decorations)");
      console.log("  - Window 4: Transparent + Frameless overlay");
      console.log("  - Window 5: Glass effect with transparency");
      console.log("\nNote: Transparency effects depend on compositor support.");
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

// Draw a circle with alpha
function drawCircle(windowId: number, cx: number, cy: number, radius: number, r: number, g: number, b: number, a: number) {
  for (let y = cy - radius; y <= cy + radius; y++) {
    for (let x = cx - radius; x <= cx + radius; x++) {
      const dx = x - cx;
      const dy = y - cy;
      const dist = Math.sqrt(dx * dx + dy * dy);
      if (dist <= radius && x >= 0 && x < 300 && y >= 0 && y < 200) {
        // Fade alpha at edges
        const edgeAlpha = Math.floor(a * (1 - dist / radius));
        manager.setPixelRgba(windowId, x, y, r, g, b, edgeAlpha);
      }
    }
  }
}

// Draw a shadow effect around window edges
function drawShadowEffect(windowId: number) {
  const shadowSize = 20;
  const width = 300;
  const height = 200;

  // Draw shadow on edges
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < shadowSize; x++) {
      const alpha = Math.floor((1 - x / shadowSize) * 100);
      manager.setPixelRgba(windowId, x, y, 0, 0, 0, alpha);
      manager.setPixelRgba(windowId, width - 1 - x, y, 0, 0, 0, alpha);
    }
  }
  for (let x = 0; x < width; x++) {
    for (let y = 0; y < shadowSize; y++) {
      const alpha = Math.floor((1 - y / shadowSize) * 100);
      manager.setPixelRgba(windowId, x, y, 0, 0, 0, alpha);
      manager.setPixelRgba(windowId, x, height - 1 - y, 0, 0, 0, alpha);
    }
  }
}

console.log("\nTransparent window demo running...");