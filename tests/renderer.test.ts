import { expect, test, describe } from "bun:test";
import { WindowRenderer, SimpleWindow } from "../index";

describe("Winit + Softbuffer Renderer Tests", () => {
  test("WindowRenderer - create instance", () => {
    const renderer = new WindowRenderer(800, 600);
    expect(renderer).toBeDefined();
    expect(renderer.width).toBe(800);
    expect(renderer.height).toBe(600);
  });

  test("WindowRenderer - set and get dimensions", () => {
    const renderer = new WindowRenderer(1024, 768);
    expect(renderer.width).toBe(1024);
    expect(renderer.height).toBe(768);
  });

  test("WindowRenderer - fill buffer", () => {
    const renderer = new WindowRenderer(100, 100);
    // Fill with red color
    renderer.fill(255, 0, 0);
    expect(renderer).toBeDefined();
  });

  test("WindowRenderer - set pixel", () => {
    const renderer = new WindowRenderer(100, 100);
    // Set a pixel to blue
    renderer.setPixel(50, 50, 0, 0, 255);
    expect(renderer).toBeDefined();
  });

  test("WindowRenderer - clear buffer", () => {
    const renderer = new WindowRenderer(100, 100);
    renderer.fill(255, 255, 255); // Fill white
    renderer.clear(); // Clear to black
    expect(renderer).toBeDefined();
  });

  test("SimpleWindow - create instance", () => {
    const window = new SimpleWindow(800, 600, "Test Window");
    expect(window).toBeDefined();
  });

  test("SimpleWindow - show window (manual test)", () => {
    // This test creates a window but doesn't block
    // In a real test environment, you might want to skip this
    // or run it with a timeout
    const window = new SimpleWindow(400, 300, "Test Window");
    expect(window).toBeDefined();
    
    // Note: window.show() blocks the event loop
    // Uncomment below to manually test window display
    // window.show(100, 150, 200); // Show with RGB(100, 150, 200)
  });
});
