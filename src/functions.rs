// GPUI Functions implementation for NAPI
use napi_derive::napi;
use crate::structs::{Pixels, DivElement, SvgElement, ImageElement, Point, Size, TextElement, ListElement, UniformListElement};

#[napi]
pub fn px(value: f64) -> Pixels {
    Pixels { value }
}

#[napi]
pub fn rems(value: f64) -> f64 {
    value 
}

#[napi]
pub fn percentage(value: f64) -> f64 {
    value
}

#[napi]
pub fn div() -> DivElement {
    DivElement::new()
}

#[napi]
pub fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

#[napi]
pub fn size(width: f64, height: f64) -> Size {
    Size { width, height }
}

#[napi]
pub fn svg() -> SvgElement {
    SvgElement::new()
}

#[napi]
pub fn img() -> ImageElement {
    ImageElement::new()
}

#[napi]
pub fn canvas() -> DivElement {
    DivElement::new()
}

#[napi]
pub fn text(_content: String) -> TextElement {
    TextElement::new()
}

#[napi]
pub fn list() -> ListElement {
    ListElement::new()
}

#[napi]
pub fn uniform_list() -> UniformListElement {
    UniformListElement::new()
}

#[napi]
pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> crate::structs::Rgba {
    crate::structs::Rgba::new(r, g, b, a)
}

#[napi]
pub fn hsla(h: f64, s: f64, l: f64, a: f64) -> crate::structs::Hsla {
    crate::structs::Hsla::new(h, s, l, a)
}

#[napi]
pub fn white() -> crate::structs::Rgba {
    crate::structs::Rgba::new(1.0, 1.0, 1.0, 1.0)
}

#[napi]
pub fn black() -> crate::structs::Rgba {
    crate::structs::Rgba::new(0.0, 0.0, 0.0, 1.0)
}

#[napi]
pub fn red() -> crate::structs::Rgba {
    crate::structs::Rgba::new(1.0, 0.0, 0.0, 1.0)
}

#[napi]
pub fn green() -> crate::structs::Rgba {
    crate::structs::Rgba::new(0.0, 1.0, 0.0, 1.0)
}

#[napi]
pub fn blue() -> crate::structs::Rgba {
    crate::structs::Rgba::new(0.0, 0.0, 1.0, 1.0)
}

#[napi]
pub fn yellow() -> crate::structs::Rgba {
    crate::structs::Rgba::new(1.0, 1.0, 0.0, 1.0)
}

#[napi]
pub fn transparent() -> crate::structs::Rgba {
    crate::structs::Rgba::new(0.0, 0.0, 0.0, 0.0)
}