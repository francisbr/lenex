use pyo3::prelude::*;

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

impl From<PyLenex> for ::lenex::Lenex {
    fn from(value: PyLenex) -> Self {
        Self {
            version: value.version,
            constructor: Python::attach(|py| (&*value.constructor.borrow(py)).into()),
        }
    }
}

impl From<::lenex::Lenex> for PyLenex {
    fn from(value: ::lenex::Lenex) -> Self {
        Self {
            version: value.version,
            constructor: Python::attach(|py| {
                Py::new(py, PyConstructor::from(value.constructor))
                    .expect("failed to allocate Constructor")
            }),
        }
    }
}
