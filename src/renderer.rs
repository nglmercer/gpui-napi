use gpui::{div, Div, ParentElement, Styled, px, Rgba as GRgba};
use crate::structs::{DivElement, Child};
use crate::enums::FlexDirection;

pub fn render_div(el: &DivElement) -> Div {
    let mut div = div();

    // Basic layout
    if el.flex {
        div = div.flex();
        if let Some(dir) = el.flex_direction {
            match dir {
                FlexDirection::Row => div = div.flex_row(),
                FlexDirection::Column => div = div.flex_col(),
                FlexDirection::RowReverse => div = div.flex_row_reverse(),
                FlexDirection::ColumnReverse => div = div.flex_col_reverse(),
            }
        }
    }

    // Styles
    if let Some(w) = el.width {
        if w == 100.0 {
            div = div.w_full();
        } else {
            div = div.w(px(w as f32));
        }
    }
    if let Some(h) = el.height {
        if h == 100.0 {
            div = div.h_full();
        } else {
            div = div.h(px(h as f32));
        }
    }

    if let Some(ref bg) = el.background {
        if let Ok(color) = parse_color(bg) {
            div = div.bg(color);
        }
    }

    if let Some(p) = el.padding {
        div = div.p(px(p as f32));
    }
    if let Some(m) = el.margin {
        div = div.m(px(m as f32));
    }
    if let Some(b) = el.border_width {
        div = div.border(px(b as f32));
    }
    if let Some(ref bc) = el.border_color {
        if let Ok(color) = parse_color(bc) {
            div = div.border_color(color);
        }
    }
    if let Some(r) = el.corner_radius {
        div = div.rounded(px(r as f32));
    }

    // Children
    for child in &el.children {
        match child {
            Child::Text(text) => {
                div = div.child(text.clone());
            }
            Child::Element(element) => {
                div = div.child(render_div(element));
            }
        }
    }

    div
}

fn parse_color(color: &str) -> Result<gpui::Rgba, String> {
    if color.starts_with('#') {
        let hex = &color[1..];
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
            return Ok(GRgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 });
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
            let a = u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?;
            return Ok(GRgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0 });
        }
    }
    
    // Default to white if parsing fails for now
    Ok(GRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })
}
