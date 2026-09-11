use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Lenex(lenex::Lenex);

#[napi]
impl Lenex {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)] // JS calls `new Lenex()`; a Rust `Default` would be dead code.
    pub fn new() -> Self {
        Lenex(lenex::Lenex::new())
    }

    #[napi(factory)]
    pub fn open(path: String) -> Result<Self> {
        Ok(Lenex(lenex::Lenex::open(path).map_err(error::to_napi)?))
    }

    #[napi(factory, js_name = "fromLefBuffer")]
    pub fn from_lef_buffer(data: Buffer) -> Result<Self> {
        Ok(Lenex(
            lenex::Lenex::from_lef_bytes(&data).map_err(error::to_napi)?,
        ))
    }

    #[napi(factory, js_name = "fromLxfBuffer")]
    pub fn from_lxf_buffer(data: Buffer) -> Result<Self> {
        Ok(Lenex(
            lenex::Lenex::from_lxf_bytes(&data).map_err(error::to_napi)?,
        ))
    }

    #[napi]
    pub fn save(&self, path: String) -> Result<()> {
        self.0.save(path).map_err(error::to_napi)
    }

    #[napi(js_name = "toLefBuffer")]
    pub fn to_lef_buffer(&self) -> Result<Buffer> {
        Ok(self
            .0
            .to_lef_string()
            .map_err(error::to_napi)?
            .into_bytes()
            .into())
    }

    #[napi(js_name = "toLxfBuffer")]
    pub fn to_lxf_buffer(&self) -> Result<Buffer> {
        Ok(self.0.to_lxf_bytes().map_err(error::to_napi)?.into())
    }

    #[napi(js_name = "toLefString")]
    pub fn to_lef_string(&self) -> Result<String> {
        self.0.to_lef_string().map_err(error::to_napi)
    }
}

// Orphan rule: both `lenex::Error` and `napi::Error` are foreign, so no `From` impl.
mod error {
    pub(super) fn to_napi(err: lenex::Error) -> napi::Error {
        napi::Error::new(napi::Status::GenericFailure, err.to_string())
    }
}
