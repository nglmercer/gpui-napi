import { WindowManager } from "../index";

// Demo: Interactive drawing canvas with shapes and patterns
console.log("Creating WindowManager for drawing demo...");
const manager = new WindowManager();

// Start the event loop
manager.start();

// Create a canvas window
const width = 500;
const height = 500;
const windowId = Number(manager.createWindow(width, height, "Drawing Canvas Demo"));
console.log(`Canvas window created with ID: ${windowId}`);

// Clear to dark gray background
for (let y = 0; y < height; y++) {
  for (let x = 0; x < width; x++) {
    manager.setPixel(windowId, x, y, 30, 30, 30);
  }
}

// Draw a color palette at the top
const paletteY = 20;
const colors = [
  { r: 255, g: 0, b: 0 },     // Red
  { r: 255, g: 165, b: 0 },   // Orange
  { r: 255, g: 255, b: 0 },   // Yellow
  { r: 0, g: 255, b: 0 },     // Green
  { r: 0, g: 0, b: 255 },     // Blue
  { r: 75, g: 0, b: 130 },    // Indigo
  { r: 238, g: 130, b: 238 }, // Violet
  { r: 255, g: 255, b: 255 }, // White
];

for (let i = 0; i < colors.length; i++) {
  const x = 30 + i * 55;
  drawRect(x - 15, paletteY - 10, 30, 20, colors[i].r, colors[i].g, colors[i].b);
}

// Draw geometric patterns

// 1. Checkerboard pattern (bottom left)
const checkerSize = 25;
const checkerX = 50;
const checkerY = 350;
for (let row = 0; row < 4; row++) {
  for (let col = 0; col < 4; col++) {
    const isWhite = (row + col) % 2 === 0;
    const color = isWhite ? 255 : 100;
    drawRect(
      checkerX + col * checkerSize,
      checkerY + row * checkerSize,
      checkerSize - 2,
      checkerSize - 2,
      color, color, color
    );
  }
}

// 2. Concentric circles (center)
const centerX = 250;
const centerY = 250;
for (let r = 100; r > 0; r -= 15) {
  const hue = (r * 2.5) % 360;
  const rgb = hsvToRgb(hue / 360, 1, 1);
  drawCircleOutline(centerX, centerY, r, rgb.r, rgb.g, rgb.b);
}

// 3. Gradient rectangle (right side)
const gradX = 350;
const gradY = 100;
const gradW = 100;
const gradH = 150;
for (let y = 0; y < gradH; y++) {
  const ratio = y / gradH;
  const r = Math.floor(255 * (1 - ratio));
  const g = Math.floor(255 * ratio);
  const b = 128;
  for (let x = 0; x < gradW; x++) {
    manager.setPixel(windowId, gradX + x, gradY + y, r, g, b);
  }
}

// 4. Star pattern (top right)
const starX = 400;
const starY = 80;
drawStar(starX, starY, 25, 5, 255, 215, 0);

// 5. Grid lines
for (let i = 0; i < width; i += 50) {
  // Vertical lines
  for (let y = 0; y < height; y++) {
    if (y > 60) { // Skip palette area
      manager.setPixel(windowId, i, y, 50, 50, 50);
    }
  }
}
for (let j = 60; j < height; j += 50) {
  // Horizontal lines
  for (let x = 0; x < width; x++) {
    manager.setPixel(windowId, x, j, 50, 50, 50);
  }
}

// 6. Random scatter points (bottom right)
for (let i = 0; i < 100; i++) {
  const x = 350 + Math.floor(Math.random() * 120);
  const y = 350 + Math.floor(Math.random() * 120);
  const r = Math.floor(Math.random() * 255);
  const g = Math.floor(Math.random() * 255);
  const b = Math.floor(Math.random() * 255);
  manager.setPixel(windowId, x, y, r, g, b);
  // Make it a small 2x2 dot
  manager.setPixel(windowId, x + 1, y, r, g, b);
  manager.setPixel(windowId, x, y + 1, r, g, b);
  manager.setPixel(windowId, x + 1, y + 1, r, g, b);
}

// Present the drawing
manager.present(windowId);
console.log("Drawing complete! Canvas displayed.");

// Add a simple animation to show interactivity
let pulse = 0;
const pulseInterval = setInterval(() => {
  pulse++;
  
  // Pulse the star
  const size = 20 + Math.floor(Math.sin(pulse * 0.2) * 5);
  
  // Clear star area
  for (let y = starY - 30; y < starY + 30; y++) {
    for (let x = starX - 30; x < starX + 30; x++) {
      if (x >= 0 && x < width && y >= 0 && y < height) {
        manager.setPixel(windowId, x, y, 30, 30, 30);
      }
    }
  }
  
  // Redraw star with new size
  drawStar(starX, starY, size, 5, 255, 215, 0);
  manager.present(windowId);
  
  if (pulse >= 30) {
    clearInterval(pulseInterval);
    console.log("Canvas demo complete!");
  }
}, 100);

// Helper functions
function drawRect(x: number, y: number, w: number, h: number, r: number, g: number, b: number) {
  for (let dy = 0; dy < h; dy++) {
    for (let dx = 0; dx < w; dx++) {
      const px = x + dx;
      const py = y + dy;
      if (px >= 0 && px < width && py >= 0 && py < height) {
        manager.setPixel(windowId, px, py, r, g, b);
      }
    }
  }
}

function drawCircleOutline(cx: number, cy: number, radius: number, r: number, g: number, b: number) {
  for (let angle = 0; angle < 360; angle += 2) {
    const rad = (angle * Math.PI) / 180;
    const x = Math.floor(cx + Math.cos(rad) * radius);
    const y = Math.floor(cy + Math.sin(rad) * radius);
    if (x >= 0 && x < width && y >= 0 && y < height) {
      manager.setPixel(windowId, x, y, r, g, b);
    }
  }
}

function drawStar(cx: number, cy: number, outerRadius: number, points: number, r: number, g: number, b: number) {
  const innerRadius = outerRadius * 0.4;
  
  for (let i = 0; i < points * 2; i++) {
    const angle = (i * Math.PI) / points - Math.PI / 2;
    const radius = i % 2 === 0 ? outerRadius : innerRadius;
    const x = Math.floor(cx + Math.cos(angle) * radius);
    const y = Math.floor(cy + Math.sin(angle) * radius);
    
    // Draw line from center to point
    const nextAngle = ((i + 1) * Math.PI) / points - Math.PI / 2;
    const nextRadius = (i + 1) % 2 === 0 ? outerRadius : innerRadius;
    const nextX = Math.floor(cx + Math.cos(nextAngle) * nextRadius);
    const nextY = Math.floor(cy + Math.sin(nextAngle) * nextRadius);
    
    drawLine(x, y, nextX, nextY, r, g, b);
  }
}

function drawLine(x0: number, y0: number, x1: number, y1: number, r: number, g: number, b: number) {
  const dx = Math.abs(x1 - x0);
  const dy = Math.abs(y1 - y0);
  const sx = x0 < x1 ? 1 : -1;
  const sy = y0 < y1 ? 1 : -1;
  let err = dx - dy;
  
  while (true) {
    if (x0 >= 0 && x0 < width && y0 >= 0 && y0 < height) {
      manager.setPixel(windowId, x0, y0, r, g, b);
    }
    
    if (x0 === x1 && y0 === y1) break;
    
    const e2 = 2 * err;
    if (e2 > -dy) {
      err -= dy;
      x0 += sx;
    }
    if (e2 < dx) {
      err += dx;
      y0 += sy;
    }
  }
}

function hsvToRgb(h: number, s: number, v: number): { r: number; g: number; b: number } {
  let r = 0, g = 0, b = 0;
  const i = Math.floor(h * 6);
  const f = h * 6 - i;
  const p = v * (1 - s);
  const q = v * (1 - f * s);
  const t = v * (1 - (1 - f) * s);
  
  switch (i % 6) {
    case 0: r = v; g = t; b = p; break;
    case 1: r = q; g = v; b = p; break;
    case 2: r = p; g = v; b = t; break;
    case 3: r = p; g = q; b = v; break;
    case 4: r = t; g = p; b = v; break;
    case 5: r = v; g = p; b = q; break;
  }
  
  return {
    r: Math.floor(r * 255),
    g: Math.floor(g * 255),
    b: Math.floor(b * 255)
  };
}

console.log("Main script completed - canvas is displayed!");
