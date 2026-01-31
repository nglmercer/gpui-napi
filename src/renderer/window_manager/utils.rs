use napi::bindgen_prelude::*;
use napi::JsNumber;

/// Convert a JavaScript number to u64
pub fn js_number_to_u64(n: JsNumber) -> Result<u64> {
    let val = n.get_double()? as u64;
    Ok(val)
}
