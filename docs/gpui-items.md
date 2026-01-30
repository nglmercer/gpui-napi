# GPUI 0.2.2 Reference Guide 🚀

This document provides an organized, high-level overview of the items available in the [GPUI crate](https://docs.rs/gpui/0.2.2/gpui/all.html). GPUI is a highly-concurrent, GPU-accelerated UI framework for Rust, built by the Zed team.

---

## 🏛️ Core Architecture

The foundations of any GPUI application.

### Application & Windows

| Item                  | Description                                                                          |
| :-------------------- | :----------------------------------------------------------------------------------- |
| **`App`**             | The main application singleton. Manages the event loop and lifecycle.                |
| **`Application`**     | Internal representation of the platform-specific app.                                |
| **`AsyncApp`**        | A handle to the app that can be sent across threads to run tasks on the main thread. |
| **`Window`**          | Represents a single native OS window.                                                |
| **`WindowHandle<V>`** | A strongly-typed handle to a window containing a specific root view.                 |
| **`WindowOptions`**   | Configuration for window creation (size, title, decorations, etc.).                  |

### Contexts

| Item                     | Description                                                                                 |
| :----------------------- | :------------------------------------------------------------------------------------------ |
| **`AppContext`**         | Shared state access. Used for spawning tasks, reading/writing globals, and managing models. |
| **`VisualContext`**      | Extension of `AppContext` for UI items. Used to manage focus and window-specific state.     |
| **`AsyncWindowContext`** | Window context that can be used from background threads.                                    |

---

## 🏗️ UI Building Blocks (Elements)

Elements are the "atoms" of the UI that are rendered every frame.

### Primitive Elements

- **`Div`**: The Swiss Army knife of layout. Supports Flexbox, borders, backgrounds, and event handling.
- **`Img`**: Renders bitmap images.
- **`Svg`**: Renders scalable vector graphics.
- **`Canvas`**: Allows for custom low-level painting using a `PathBuilder`.
- **`List` / `UniformList`**: High-performance lists for rendering large numbers of items efficiently.

### Element Traits

- **`Element`**: The base trait for anything that can be laid out and painted.
- **`IntoElement`**: Conversion trait to turn a type into an element.
- **`Render`**: Trait for views that define how they should be rendered into an `AnyElement`.

---

## 🎨 Styling & Layout

GPUI uses a Flexbox-based layout system with CSS-like properties.

### Sizing Units

- **`Pixels` (px)**: Physical or logical pixels.
- **`Rems` (rems)**: Relative to the base font size.
- **`Percentage` (%)**: Relative to the parent container.
- **`DevicePixels`**: Physical pixels on the display.

### Decorative Types

- **`Hsla` / `Rgba`**: Color representations.
- **`BoxShadow`**: Customizable drop shadows.
- **`HighlightStyle`**: Used for text highlighting/selection.
- **`Fill` / `Stroke`**: Instructions for how to paint paths and shapes.

---

## 📦 State Management

GPUI uses an "Entity-Component-System" like approach for state.

- **`Entity<T>`**: A reference-counted wrapper around state (`T`) that can be shared and observed.
- **`View<V>`**: An entity that can be rendered. Implements `Render`.
- **`Model<M>`**: An entity that represents pure data/logic without a visual component.
- **`Subscription`**: Returned when observing an entity; keeps the observation active until dropped.

---

## ⌨️ Events & Interactivity

Handling user input and system events.

### Input Events

- **`ClickEvent`**: Triggered on mouse clicks.
- **`KeyDownEvent` / `KeyUpEvent`**: Keyboard interaction.
- **`MouseMoveEvent` / `MouseDownEvent`**: Granular mouse tracking.
- **`ScrollWheelEvent`**: Handling mouse or trackpad scrolling.

### Focus & Keymap

- **`FocusHandle`**: Manages which element currently receives keyboard input.
- **`Keymap`**: Maps keystrokes to `Action`s.
- **`Action`**: A trait representing a high-level command (e.g., `Save`, `Quit`).
- **`Keystroke`**: Represents a specific key combination (e.g., `cmd-s`).

---

## 🧵 Concurrency & Tasks

GPUI is designed for highly parallel workloads.

- **`Task<T>`**: A handle to a background operation.
- **`BackgroundExecutor`**: Used to spawn heavy compute tasks off the main thread.
- **`ForegroundExecutor`**: Used to schedule tasks back onto the main UI thread.
- **`Timer` / `Timeout`**: Async timing utilities.

---

## 📖 Complete Item Reference (Categorized)

### Structs

<details>
<summary>Expand All Structs</summary>

- `Anchored`, `AnchoredState`, `Animation`, `AnimationElement`, `AnyDrag`, `AnyElement`, `AnyEntity`, `AnyImageCache`, `AnyTooltip`, `AnyView`, `AnyWeakEntity`, `AnyWeakView`, `AnyWindowHandle`, `App`, `Application`, `ArenaClearNeeded`, `AsyncApp`, `AsyncWindowContext`, `Background`, `BackgroundExecutor`, `BindingIndex`, `Boundary`, `Bounds`, `BoundsRefinement`, `BoxShadow`, `Canvas`, `Capslock`, `Cascade`, `CascadeSlot`, `ClipboardItem`, `ClipboardString`, `ContentMask`, `Context`, `ContextEntry`, `Corners`, `CornersRefinement`, `DebugBelow`, `DecorationRun`, `Deferred`, `DeferredScrollToItem`, `DevicePixels`, `DismissEvent`, `DisplayId`, `Div`, `DivFrameState`, `DivInspectorState`, `DragMoveEvent`, `Drawable`, `DummyKeyboardMapper`, `Edges`, `EdgesRefinement`, `ElementClickedState`, `ElementInputHandler`, `Empty`, `EmptyView`, `Entity`, `EntityId`, `ExternalPaths`, `FallbackPromptRenderer`, `FillOptions`, `FocusHandle`, `FocusId`, `FocusOutEvent`, `Font`, `FontFallbacks`, `FontFamilyId`, `FontFeatures`, `FontId`, `FontMetrics`, `FontRun`, `FontWeight`, `ForegroundExecutor`, `GlobalElementId`, `GlyphId`, `GpuSpecs`, `GpuiBorrow`, `GridLocation`, `GroupStyle`, `HighlightStyle`, `Hitbox`, `HitboxId`, `Hsla`, `Image`, `ImageCacheElement`, `ImageFormatIter`, `ImageId`, `ImageStyle`, `Img`, `ImgLayoutState`, `Inspector`, `InspectorElementId`, `InspectorElementPath`, `InteractiveElementState`, `InteractiveText`, `Interactivity`, `InvalidKeystrokeError`, `ItemSize`, `KeyBinding`, `KeyBindingMetaIndex`, `KeyContext`, `KeyDownEvent`, `KeyUpEvent`, `KeybindingKeystroke`, `KeyboardClickEvent`, `Keymap`, `KeymapVersion`, `Keystroke`, `KeystrokeEvent`, `LayoutId`, `LineLayout`, `LineWrapper`, `LineWrapperHandle`, `LinearColorStop`, `List`, `ListOffset`, `ListPrepaintState`, `ListScrollEvent`, `ListState`, `Menu`, `Modifiers`, `ModifiersChangedEvent`, `MouseClickEvent`, `MouseDownEvent`, `MouseExitEvent`, `MouseMoveEvent`, `MouseUpEvent`, `NoAction`, `OsMenu`, `OwnedMenu`, `OwnedOsMenu`, `PaintQuad`, `Path`, `PathBuilder`, `PathPromptOptions`, `Percentage`, `Pixels`, `Point`, `PointRefinement`, `PromptHandle`, `PromptResponse`, `Radians`, `Rems`, `RenderImage`, `RenderablePromptHandle`, `Reservation`, `RetainAllImageCache`, `RetainAllImageCacheProvider`, `Rgba`, `ScaledPixels`, `Scope`, `ScreenCaptureFrame`, `ScrollAnchor`, `ScrollHandle`, `ScrollWheelEvent`, `SemanticVersion`, `ShapedGlyph`, `ShapedLine`, `ShapedRun`, `SharedString`, `SharedUri`, `Size`, `SizeRefinement`, `SourceMetadata`, `Stateful`, `StrikethroughStyle`, `StrikethroughStyleRefinement`, `StrokeOptions`, `Style`, `StyleRefinement`, `StyledText`, `Subscription`, `Surface`, `Svg`, `SystemWindowTabController`, `Task`, `TaskLabel`, `TextLayout`, `TextRun`, `TextStyle`, `TextStyleRefinement`, `TextSystem`, `Tiling`, `Timeout`, `Timer`, `TitlebarOptions`, `TooltipId`, `Transformation`, `TransformationMatrix`, `UTF16Selection`, `UnderlineStyle`, `UnderlineStyleRefinement`, `UniformList`, `UniformListFrameState`, `UniformListScrollHandle`, `UniformListScrollState`, `WeakEntity`, `WeakFocusHandle`, `Window`, `WindowControls`, `WindowHandle`, `WindowId`, `WindowOptions`, `WindowTextSystem`, `WrapBoundary`, `WrappedLine`, `WrappedLineLayout`
</details>

### Enums

<details>
<summary>Expand All Enums</summary>

- `AbsoluteLength`, `ActionBuildError`, `AlignContent`, `AlignItems`, `AnchoredFitMode`, `AnchoredPositionMode`, `ArcCow`, `AssetLogger`, `AvailableSpace`, `Axis`, `BorderStyle`, `ClickEvent`, `ClipboardEntry`, `ColorSpace`, `Corner`, `CursorStyle`, `Decorations`, `DefiniteLength`, `DispatchPhase`, `Display`, `ElementId`, `FileDropEvent`, `Fill`, `FillRule`, `FlexDirection`, `FlexWrap`, `FontStyle`, `GridPlacement`, `HitboxBehavior`, `ImageAssetLoader`, `ImageCacheError`, `ImageCacheItem`, `ImageFormat`, `ImageSource`, `KeyBindingContextPredicate`, `KeyboardButton`, `Length`, `LineFragment`, `ListAlignment`, `ListHorizontalSizingBehavior`, `ListMeasuringBehavior`, `ListSizingBehavior`, `MenuItem`, `MouseButton`, `NavigationDirection`, `ObjectFit`, `OsAction`, `Overflow`, `OwnedMenuItem`, `PathStyle`, `PlatformInput`, `Position`, `PromptButton`, `PromptLevel`, `ResizeEdge`, `Resource`, `ScrollDelta`, `ScrollStrategy`, `SurfaceSource`, `SystemMenuType`, `TextAlign`, `TextOverflow`, `TouchPhase`, `Visibility`, `WhiteSpace`, `WindowAppearance`, `WindowBackgroundAppearance`, `WindowBounds`, `WindowControlArea`, `WindowDecorations`, `WindowKind`
</details>

### Traits

<details>
<summary>Expand All Traits</summary>

- `Action`, `Along`, `AnimationExt`, `AppContext`, `AsKeystroke`, `Asset`, `AssetSource`, `BorrowAppContext`, `Element`, `EntityInputHandler`, `EventEmitter`, `Flatten`, `Focusable`, `FutureExt`, `Global`, `Half`, `ImageCache`, `ImageCacheProvider`, `InputEvent`, `InputHandler`, `InteractiveElement`, `IntoElement`, `IsEmpty`, `IsZero`, `KeyEvent`, `ManagedView`, `MouseEvent`, `Negate`, `ParentElement`, `PlatformDisplay`, `PlatformKeyboardLayout`, `PlatformKeyboardMapper`, `Prompt`, `ReadGlobal`, `Refineable`, `Render`, `RenderOnce`, `ScreenCaptureSource`, `ScreenCaptureStream`, `StatefulInteractiveElement`, `Styled`, `StyledImage`, `UniformListDecoration`, `UpdateGlobal`, `VisualContext`
</details>

### Functions

<details>
<summary>Expand All Functions</summary>

- `anchored`, `auto`, `background_executor`, `black`, `blue`, `bounce`, `bounds`, `canvas`, `combine_highlights`, `deferred`, `div`, `ease_in_out`, `ease_out_quint`, `fallback_prompt_renderer`, `fill`, `font`, `generate_list_of_all_registered_actions`, `green`, `guess_compositor`, `hash`, `hsla`, `image_cache`, `img`, `is_no_action`, `linear`, `linear_color_stop`, `linear_gradient`, `list`, `opaque_grey`, `outline`, `pattern_slash`, `percentage`, `phi`, `point`, `pulsating_between`, `px`, `quad`, `quadratic`, `radians`, `red`, `relative`, `rems`, `retain_all`, `rgb`, `rgba`, `size`, `solid_background`, `surface`, `svg`, `transparent_black`, `transparent_white`, `uniform_list`, `white`, `yellow`
</details>

### Macros

<details>
<summary>Expand All Macros</summary>

- `actions`, `border_style_methods`, `box_shadow_style_methods`, `cursor_style_methods`, `margin_style_methods`, `overflow_style_methods`, `padding_style_methods`, `position_style_methods`, `register_action`, `visibility_style_methods`, `ctor`, `test`
</details>
