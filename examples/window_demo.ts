import { SimpleWindow, WindowRenderer } from "../index";

// Demo 1: Simple colored window
console.log("Creating a simple window...");
const simpleWindow = new SimpleWindow(800, 600, "Winit + Softbuffer Demo");
console.log("Window created. Showing with blue color...");

// Show the window with a blue background
// Note: This will block until the window is closed
simpleWindow.show(100, 150, 255);

// Demo 2: WindowRenderer with pixel manipulation (uncomment to test)

console.log("Creating WindowRenderer...");
const renderer = new WindowRenderer(400, 400);

// Fill with black
renderer.clear();

// Draw a red diagonal line
for (let i = 0; i < 400; i++) {
  renderer.setPixel(i, i, 255, 0, 0);
}

// Draw a green horizontal line
for (let x = 50; x < 350; x++) {
  renderer.setPixel(x, 200, 0, 255, 0);
}

// Draw a blue vertical line
for (let y = 50; y < 350; y++) {
  renderer.setPixel(200, y, 0, 0, 255);
}

console.log("Pixel buffer prepared.");

// Present the rendered pixels in a window
console.log("Opening window with rendered pixels...");
renderer.present("Pixel Renderer Demo");

console.log("Window closed.");
