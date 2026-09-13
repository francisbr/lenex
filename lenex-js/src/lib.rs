// Load-bearing, not convenience: without a reference to the `lenex` rlib the linker drops it and the .node registers nothing.
pub use ::lenex::Lenex;

// `impl From<lenex::Error> for napi::Error` is illegal here: both types are foreign to this crate.
// Convert per call site instead:
// `.map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))`
