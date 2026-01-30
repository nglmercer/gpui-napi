# gpui-napi Implementation Plan

This document outlines the tasks required to implement functional GPUI bindings for Node.js using NAPI-RS.

## Phase 1: Core Architecture (Fundamental)

- [ ] Implement `AppHandle` and its connection to `gpui::App`.
- [ ] Implement `WindowOptions` mapping to `gpui::WindowOptions`.
- [ ] Implement `WindowHandle` and `open_window(options)`.
- [ ] Bridge `AppContext` and `VisualContext`.

## Phase 2: Fundamental Elements & Layout

- [ ] Implement `Pixels`, `Rems`, `Percentage` correctly.
- [ ] Implement `Hsla` and `Rgba` colors with proper conversions.
- [ ] Implement `DivElement` with full GPUI `Div` methods (borders, padding, margins, flexbox).
- [ ] Implement `text` element and font configuration.

## Phase 3: Assets & Graphics

- [ ] Implement `Img` element for bitmap images.
- [ ] Implement `Svg` element for vector graphics.
- [ ] Implement `Canvas` element for low-level painting.

## Phase 4: State Management & Views

- [ ] Implement `Model` (wrapper around `gpui::Model`).
- [ ] Implement `View` (wrapper around `gpui::View`).
- [ ] Implement subscriptions and event emitters back to JS.

## Phase 5: Interactivity & Events

- [ ] Implement event listeners on `DivElement` (`on_click`, `on_mouse_down`, etc.).
- [ ] Map GPUI events to JS objects.
- [ ] Focus management (`FocusHandle`).
- [ ] Keyboard shortcuts and Actions.

## Phase 6: Lists & Performance

- [ ] Implement `List` element.
- [ ] Implement `UniformList` for large datasets.

## Phase 7: Concurrency

- [ ] Bridge GPUI `Task` and `Timer` with JS Promises/Async.
- [ ] Support background tasks.
