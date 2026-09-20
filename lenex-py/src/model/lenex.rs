use pyo3::prelude::*;

use ::lenex::prelude::{Constructor, Lenex};

use crate::model::constructor::PyConstructor;

#[pyclass(name = "Lenex", get_all, set_all)]
pub struct PyLenex {
    version: String,
    constructor: Py<PyConstructor>,
}

#[pymethods]
impl PyLenex {
    #[new]
    fn new(version: String, constructor: Py<PyConstructor>) -> Self {
        Self {
            version,
            constructor,
        }
    }
}

impl From<PyLenex> for Lenex {
    fn from(value: PyLenex) -> Self {
        Lenex::builder(
            value.version,
            Python::attach(|py| -> Constructor { (&*value.constructor.borrow(py)).into() }),
        )
        .build()
    }
}

impl From<Lenex> for PyLenex {
    fn from(value: Lenex) -> Self {
        Self {
            version: value.version,
            constructor: Python::attach(|py| {
                Py::new(py, PyConstructor::from(value.constructor))
                    .expect("failed to allocate Constructor")
            }),
        }
    }
}
