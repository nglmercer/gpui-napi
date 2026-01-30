// GPUI Enums implementation for NAPI
use napi_derive::napi;

#[napi]
pub enum Visibility {
    Visible,
    Hidden,
}

#[napi]
pub enum AlignContent {
    Start,
    End,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

#[napi]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[napi]
#[derive(Clone, Copy)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[napi]
pub enum Display {
    Flex,
    None,
}

#[napi]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[napi]
pub enum Position {
    Relative,
    Absolute,
}

#[napi]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[napi]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
    None,
}

#[napi]
pub enum CursorStyle {
    Default,
    Pointer,
    Text,
    Wait,
    Help,
    NotAllowed,
    ContextSMenu,
    Cell,
    Crosshair,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ColResize,
    RowResize,
    NResize,
    EResize,
    SResize,
    WResize,
    NeResize,
    NwResize,
    SeResize,
    SwResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
}

#[napi]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[napi]
pub enum ListAlignment {
    Top,
    Center,
    Bottom,
}

#[napi]
pub enum WindowKind {
    Normal,
    PopUp,
    ToolTip,
}

#[napi]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[napi]
pub enum AvailableSpace {
    Definite,
    MinContent,
    MaxContent,
}

#[napi]
pub enum ColorSpace {
    Srgb,
    Oklab,
}

#[napi]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[napi]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[napi]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other,
}

#[napi]
pub enum NavigationDirection {
    Next,
    Prev,
    Up,
    Down,
    Left,
    Right,
}

#[napi]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[napi]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Canceled,
}

#[napi]
pub enum WhiteSpace {
    Normal,
    Nowrap,
    Pre,
    PreLine,
    PreWrap,
}

#[napi]
pub enum ListSizingBehavior {
    Fixed,
    Auto,
}

#[napi]
pub enum ImageFormat {
    Rgba8,
    Bgra8,
    R16F,
}

#[napi]
pub enum PlatformInput {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp,
    MouseMove,
    MouseExited,
    ScrollWheel,
}

#[napi]
pub enum ImageSource {
    Uri,
    File,
    Data,
}

#[napi]
pub enum FileDropEvent {
    Entered,
    Hovered,
    Exited,
    Dropped,
}

#[napi]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[napi]
pub enum AnchoredFitMode {
    Overflow,
    Snap,
}

#[napi]
pub enum AnchoredPositionMode {
    Absolute,
    ToWindow,
}

#[napi]
pub enum ResizeEdge {
    Top,
    Right,
    Bottom,
    Left,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

#[napi]
pub enum PromptLevel {
    Info,
    Warning,
    Critical,
}

#[napi]
pub enum DispatchPhase {
    Capture,
    Bubble,
}

#[napi]
pub enum Decorations {
    Server,
    Client,
}

#[napi]
pub enum WindowAppearance {
    Light,
    Dark,
    VibrantLight,
    VibrantDark,
}

#[napi]
pub enum WindowBackgroundAppearance {
    Opaque,
    Transparent,
    Blurred,
}

#[napi]
pub enum WindowDecorations {
    Full,
    None,
}

#[napi]
pub enum WindowControlArea {
    TopLeft,
    TopRight,
}

#[napi]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

#[napi]
pub enum HitboxBehavior {
    PassThrough,
    Block,
}

#[napi]
pub enum ListHorizontalSizingBehavior {
    Unconstrained,
    Constrained,
}

#[napi]
pub enum ListMeasuringBehavior {
    FixedItemSize,
    VariableItemSize,
}

#[napi]
pub enum SystemMenuType {
    App,
    File,
    Edit,
    View,
    Window,
    Help,
}






