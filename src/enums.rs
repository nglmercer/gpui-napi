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



