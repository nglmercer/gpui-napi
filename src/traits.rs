// GPUI Traits interfaces for NAPI
use napi_derive::napi;

// These are shell representations of GPUI traits that will be used to 
// enforce interfaces or provide common functionality in JS.

#[napi]
pub struct RenderTrait {
    // Shell for Render trait implementations
}

#[napi]
pub struct ActionTrait {
    // Shell for Action trait implementations
}

#[napi]
pub struct StyledTrait {
    // Shell for Styled trait implementations
}

#[napi]
pub struct InteractiveTrait {
    // Shell for Interactive trait implementations
}
