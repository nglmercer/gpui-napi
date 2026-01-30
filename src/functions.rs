// GPUI Functions implementation for NAPI
use napi_derive::napi;
use crate::structs::{Pixels, DivElement, SvgElement, ImageElement, Point, Size};

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
    SvgElement {}
}

#[napi]
pub fn img() -> ImageElement {
    ImageElement {}
}

#[napi]
pub fn canvas() -> DivElement {
    DivElement::new()
}
