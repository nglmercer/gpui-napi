// GPUI Structs implementation for NAPI - Core Components
use napi_derive::napi;

#[napi(object)]
pub struct Pixels {
    pub value: f64,
}

#[napi(object)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[napi]
pub struct Hsla {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: f64,
}

#[napi]
impl Hsla {
    #[napi(constructor)]
    pub fn new(h: f64, s: f64, l: f64, a: f64) -> Self {
        Self { h, s, l, a }
    }
}

#[napi]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[napi]
impl Rgba {
    #[napi(constructor)]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }
}

#[napi(object)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[napi(object)]
pub struct Bounds {
    pub origin: Point,
    pub size: Size,
}

#[napi]
pub struct AppHandle {
    // Reference to gpui::App
}

#[napi]
impl AppHandle {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[napi]
    pub fn run(&self) { }

    #[napi]
    pub fn quit(&self) { }
}

#[napi]
pub struct WindowHandle {
    pub id: u32,
}

#[napi]
impl WindowHandle {
    #[napi]
    pub fn close(&self) { }

    #[napi]
    pub fn focus(&self) { }
}

#[napi]
pub struct DivElement {
    // Logic for Div
}

#[napi]
impl DivElement {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[napi]
    pub fn flex(&self) -> Self { Self {} }
    
    #[napi]
    pub fn flex_col(&self) -> Self { Self {} }

    #[napi]
    pub fn w_full(&self) -> Self { Self {} }

    #[napi]
    pub fn h_full(&self) -> Self { Self {} }

    #[napi]
    pub fn bg(&self, _color: String) -> Self { Self {} }

    #[napi]
    pub fn p(&self, _value: f64) -> Self { Self {} }

    #[napi]
    pub fn m(&self, _value: f64) -> Self { Self {} }
}

#[napi]
pub struct ImageElement {}

#[napi]
pub struct SvgElement {}

#[napi]
pub struct AppContext {}

#[napi]
pub struct VisualContext {}

#[napi(object)]
pub struct EntityId {
    pub id: i64,
}

#[napi]
pub struct Entity {
    pub id: i64,
}
