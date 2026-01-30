// GPUI Functions implementation for NAPI
use napi_derive::napi;
use crate::structs::Pixels;

#[napi]
pub fn px(value: f64) -> Pixels {
    Pixels { value }
}

#[napi]
pub fn rems(value: f64) -> f64 {
    value // Simplified for now
}

#[napi]
pub fn percentage(value: f64) -> f64 {
    value // Simplified for now
}

#[napi]
pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> crate::structs::Rgba {
    crate::structs::Rgba { r, g, b, a }
}

#[napi]
pub fn hsla(h: f64, s: f64, l: f64, a: f64) -> crate::structs::Hsla {
    crate::structs::Hsla { h, s, l, a }
}

#[napi]
pub fn div() -> crate::structs::DivElement {
    crate::structs::DivElement::new()
}

#[napi]
pub fn point(x: f64, y: f64) -> crate::structs::Point {
    crate::structs::Point { x, y }
}

#[napi]
pub fn size(width: f64, height: f64) -> crate::structs::Size {
    crate::structs::Size { width, height }
}

#[napi]
pub fn svg() -> crate::structs::DivElement {
    // Should be SvgElement but using Div for now as shell
    crate::structs::DivElement::new()
}

#[napi]
pub fn img() -> crate::structs::DivElement {
    // Should be ImgElement but using Div for now as shell
    crate::structs::DivElement::new()
}

#[napi]
pub fn canvas() -> crate::structs::DivElement {
    crate::structs::DivElement::new()
}

