use pyo3::prelude::*;

pub mod contact;

use crate::model::constructor::contact::PyContact;

#[pyclass(name = "Constructor", get_all, set_all)]
pub struct PyConstructor {
    name: String,
    version: String,
    contact: Py<PyContact>,
    registration: Option<String>,
}

#[pymethods]
impl PyConstructor {
    #[new]
    #[pyo3(signature = (name, version, contact, *, registration=None))]
    fn new(
        name: String,
        version: String,
        contact: Py<PyContact>,
        registration: Option<String>,
    ) -> Self {
        Self {
            name,
            version,
            contact,
            registration,
        }
    }
}

impl From<&PyConstructor> for ::lenex::Constructor {
    fn from(value: &PyConstructor) -> Self {
        Self {
            name: value.name.clone(),
            version: value.version.clone(),
            contact: Python::attach(|py| value.contact.borrow(py).clone().into()),
            registration: value.registration.clone(),
        }
    }
}

impl From<::lenex::Constructor> for PyConstructor {
    fn from(value: ::lenex::Constructor) -> Self {
        Self {
            name: value.name,
            version: value.version,
            contact: Python::attach(|py| {
                Py::new(py, PyContact::from(value.contact)).expect("failed to allocate Contact")
            }),
            registration: value.registration,
        }
    }
}
