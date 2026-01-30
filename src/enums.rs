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

