use pyo3::prelude::*;

pyo3::create_exception!(lenex, LenexError, pyo3::exceptions::PyException);

mod model {
    pub mod constructor;
    pub mod lenex;
}

#[pymodule]
mod lenex {
    #[pymodule_export]
    pub use super::model::constructor::PyConstructor;
    #[pymodule_export]
    pub use super::model::constructor::contact::PyContact;
    #[pymodule_export]
    pub use super::model::lenex::PyLenex;
}
