# GPUI-NAPI Implementation Roadmap 🗺️

This checklist tracks the progress of bridging [GPUI 0.2.2](https://docs.rs/gpui/0.2.2/gpui/all.html) features to Node.js via NAPI-RS.

## 🟢 Phase 1: Foundations (Complete)

- [x] **Project Structure**: Modularized Rust backend (`structs.rs`, `enums.rs`, etc.).
- [x] **Basic Scaling/Layout Units**: `Pixels`, `Rems`, `Percentage`.
- [x] **Colors**: `Hsla`, `Rgba` with property bindings.
- [x] **Essential Enums**: Flexbox alignment, visibility, display modes.
- [x] **NAPI Build Pipeline**: Configured binary naming and CI/CD targets.

---

## 🟡 Phase 2: Core Components (In Progress)

### 🏛️ Lifecycle & Windows

- [ ] **`AppHandle` Implementation**: Connect to actual `gpui::App`.
- [ ] **Window Creation**: Implement `AppHandle.open_window(options)`.
- [ ] **Context Management**: Bridge `AppContext` and `VisualContext`.
- [ ] **Window Options**: Implement full `WindowOptions` struct.

### 🏗️ UI Elements (Elements)

- [ ] **Div Implementation**: Match all GPUI `Div` methods (borders, padding, margin, etc.).
- [ ] **Opaque Elements**: Bridge `Img`, `Svg`, and `Canvas`.
- [ ] **Text Rendering**: Implement `Text`, `StyledText`, and font configuration.
- [ ] **List Systems**: Bridge `List` and `UniformList` for high-performance scrolling.

---

## 🟠 Phase 3: Interactivity & State (Next)

### ⌨️ Event System

- [ ] **Mouse Events**: `Click`, `Drag`, `Move`, `ScrollWheel`.
- [ ] **Keyboard Events**: `KeyDown`, `KeyUp` with modifier support.
- [ ] **Focus Management**: Bridge `FocusHandle` and focus lifecycle.

### 📦 State & Entities

- [ ] **Model/View System**: Wrap `gpui::Model<T>` and `gpui::View<V>`.
- [ ] **Subscriptions**: Allow JS to subscribe to state changes in Rust models.
- [ ] **Task System**: Coordinate Node.js `Promise` with GPUI `Task`.

---

## 🔴 Phase 4: Advanced Features (Planned)

### 🎨 Style System

- [ ] **Tailwind-like API**: Completion of the Fluent API for all style properties.
- [ ] **Themes**: Bridge global color and style overrides.
- [ ] **Animations**: Logic for `Animation` and micro-transitions.

### 🛠️ Tooling & DX

- [ ] **Asset Loading**: Bridge `AssetSource` for local/remote assets.
- [ ] **Macros**: Custom proc-macros for registering actions in JS.
- [ ] **TypeScript Declarations**: Fully documented `index.d.ts`.

---

## 📊 Summary of Items Status

| Categoría     | Total Items | Implemented | Progress |
| :------------ | :---------: | :---------: | :------- |
| **Structs**   |    200+     |     32      | 🟢 16%   |
| **Enums**     |     70+     |     24      | 🟢 34%   |
| **Functions** |     50+     |     12      | 🟢 24%   |
| **Traits**    |     40+     |      0      | ⚪ 0%    |
| **Macros**    |     10+     |      0      | ⚪ 0%    |

> **Note**: Implementation focuses on the "Core Architecture" and "UI Elements" identified in [gpui-items.md](./docs/gpui-items.md).
