import { WindowManager } from "../index";

// Demo: Smooth animation with bouncing ball
console.log("Creating WindowManager for animation demo...");
const manager = new WindowManager();

// Start the event loop
manager.start();

// Create a window for the animation
const width = 600;
const height = 400;
const windowId = Number(manager.createWindow(width, height, "Bouncing Ball Animation"));
console.log(`Animation window created with ID: ${windowId}`);

// Ball properties
let ballX = width / 2;
let ballY = height / 2;
let velocityX = 4;
let velocityY = 3;
const ballRadius = 15;

// Trail effect - store previous positions
const trailLength = 10;
const trail: { x: number; y: number }[] = [];

// Animation loop
let frameCount = 0;
const maxFrames = 600; // Run for ~10 seconds at 60fps

console.log("Starting animation...");

const animationInterval = setInterval(() => {
  frameCount++;
  
  // Clear to black
  manager.clearBlack(windowId);
  
  // Update ball position
  ballX += velocityX;
  ballY += velocityY;
  
  // Bounce off walls
  if (ballX - ballRadius < 0 || ballX + ballRadius >= width) {
    velocityX = -velocityX;
    ballX = Math.max(ballRadius, Math.min(width - ballRadius - 1, ballX));
  }
  if (ballY - ballRadius < 0 || ballY + ballRadius >= height) {
    velocityY = -velocityY;
    ballY = Math.max(ballRadius, Math.min(height - ballRadius - 1, ballY));
  }
  
  // Add current position to trail
  trail.unshift({ x: ballX, y: ballY });
  if (trail.length > trailLength) {
    trail.pop();
  }
  
  // Draw trail (fading effect)
  for (let i = 0; i < trail.length; i++) {
    const alpha = 1 - i / trailLength;
    const radius = Math.max(2, ballRadius * alpha * 0.5);
    const intensity = Math.floor(255 * alpha);
    drawCircle(trail[i].x, trail[i].y, radius, intensity, intensity, 0);
  }
  
  // Draw the main ball (bright white with gradient effect)
  drawCircle(ballX, ballY, ballRadius, 255, 255, 255);
  drawCircle(ballX, ballY, ballRadius - 4, 200, 200, 255);
  drawCircle(ballX, ballY, ballRadius - 8, 150, 150, 255);
  
  // Draw border
  drawBorder();
  
  // Present the frame
  manager.present(windowId);
  
  // Log progress every 60 frames
  if (frameCount % 60 === 0) {
    console.log(`Frame ${frameCount}: Ball at (${Math.floor(ballX)}, ${Math.floor(ballY)})`);
  }
  
  // Stop after max frames
  if (frameCount >= maxFrames) {
    clearInterval(animationInterval);
    console.log("Animation complete!");
    console.log(`Final ball position: (${Math.floor(ballX)}, ${Math.floor(ballY)})`);
    console.log("Window remains open.");
  }
}, 16); // ~60 FPS

// Helper function to draw a filled circle
function drawCircle(cx: number, cy: number, radius: number, r: number, g: number, b: number) {
  for (let y = -radius; y <= radius; y++) {
    for (let x = -radius; x <= radius; x++) {
      if (x * x + y * y <= radius * radius) {
        const px = Math.floor(cx + x);
        const py = Math.floor(cy + y);
        if (px >= 0 && px < width && py >= 0 && py < height) {
          manager.setPixel(windowId, px, py, r, g, b);
        }
      }
    }
  }
}

// Helper function to draw window border
function drawBorder() {
  const borderColor = { r: 100, g: 100, b: 100 };
  
  // Top and bottom borders
  for (let x = 0; x < width; x++) {
    manager.setPixel(windowId, x, 0, borderColor.r, borderColor.g, borderColor.b);
    manager.setPixel(windowId, x, height - 1, borderColor.r, borderColor.g, borderColor.b);
  }
  
  // Left and right borders
  for (let y = 0; y < height; y++) {
    manager.setPixel(windowId, 0, y, borderColor.r, borderColor.g, borderColor.b);
    manager.setPixel(windowId, width - 1, y, borderColor.r, borderColor.g, borderColor.b);
  }
}

console.log("Animation started - watch the bouncing ball!");
