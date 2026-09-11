use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(lenex, LenexError, PyException);

#[pyclass]
pub struct Lenex(::lenex::Lenex);

#[pymethods]
impl Lenex {
    #[new]
    fn new() -> Self {
        Lenex(::lenex::Lenex::new())
    }

    #[staticmethod]
    fn open(path: &str) -> PyResult<Self> {
        Ok(Lenex(::lenex::Lenex::open(path).map_err(error::to_py_err)?))
    }

    #[staticmethod]
    fn from_lef_bytes(data: &[u8]) -> PyResult<Self> {
        Ok(Lenex(
            ::lenex::Lenex::from_lef_bytes(data).map_err(error::to_py_err)?,
        ))
    }

    #[staticmethod]
    fn from_lxf_bytes(data: &[u8]) -> PyResult<Self> {
        Ok(Lenex(
            ::lenex::Lenex::from_lxf_bytes(data).map_err(error::to_py_err)?,
        ))
    }

    fn save(&self, path: &str) -> PyResult<()> {
        self.0.save(path).map_err(error::to_py_err)
    }

    fn to_lef_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self
            .0
            .to_lef_string()
            .map_err(error::to_py_err)?
            .into_bytes())
    }

    fn to_lxf_bytes(&self) -> PyResult<Vec<u8>> {
        self.0.to_lxf_bytes().map_err(error::to_py_err)
    }

    fn to_lef_string(&self) -> PyResult<String> {
        self.0.to_lef_string().map_err(error::to_py_err)
    }
}

// Orphan rule: both `lenex::Error` and `PyErr` are foreign, so no `From` impl.
mod error {
    pub(super) fn to_py_err(err: ::lenex::Error) -> pyo3::PyErr {
        super::LenexError::new_err(err.to_string())
    }
}

#[pymodule]
fn lenex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Lenex>()?;
    m.add("LenexError", m.py().get_type::<LenexError>())?;
    Ok(())
}
