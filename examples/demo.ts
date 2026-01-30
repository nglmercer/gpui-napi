import { AppHandle, div, px, white, red, blue, FlexDirection } from '../index.js';

console.log("Starting GPUI NAPI Example...");

const app = new AppHandle();

app.run(() => {
    console.log("GPUI App is running!");

    const window = app.openWindow({
        titlebar: true,
        center: true,
        focus: true,
        show: true,
        bounds: {
            origin: { x: 100, y: 100 },
            size: { width: 800, height: 600 }
        }
    });

    // Create a container with some style
    const container = div()
        .flexCol()
        .bg("#1a1a1a")
        .wFull()
        .hFull()
        .p(20)
        .gap(10);

    // Create a "button" element
    const button = div()
        .flex()
        .bg("#007acc")
        .p(10)
        .rounded(6)
        .child("Click Me")
        .onClick(() => {
            console.log("Button clicked!");
        });

    // Create a "buffer frame" placeholder (using a div with specific background)
    const bufferFrame = div()
        .flex()
        .bg("#2d2d2d")
        .border(2)
        .borderColor("#444")
        .wFull()
        .hFull()
        .rounded(4)
        .child("Buffer Frame Placeholder");

    // Nest elements
    container.child(button);
    container.child(bufferFrame);

    // Set as root
    window.setRoot(container);

    console.log("Window root set with button and buffer frame.");
});
