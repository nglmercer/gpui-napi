use napi_derive::napi;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod structs;
pub mod traits;

#[napi]
pub fn get_gpui_version() -> String {
    "0.2.2".to_string()
}
