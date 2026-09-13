use pyo3::prelude::*;

#[pyclass(name = "Contact", skip_from_py_object, get_all, set_all)]
#[derive(Clone)]
pub struct PyContact {
    email: String,
    city: Option<String>,
    country: Option<String>,
    fax: Option<String>,
    internet: Option<String>,
    name: Option<String>,
    mobile: Option<String>,
    phone: Option<String>,
    state: Option<String>,
    street: Option<String>,
    street2: Option<String>,
    zip: Option<String>,
}

#[pymethods]
impl PyContact {
    #[new]
    #[pyo3(signature = (email, *, city=None, country=None, fax=None, internet=None, name=None, mobile=None, phone=None, state=None, street=None, street2=None, zip=None))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        email: String,
        city: Option<String>,
        country: Option<String>,
        fax: Option<String>,
        internet: Option<String>,
        name: Option<String>,
        mobile: Option<String>,
        phone: Option<String>,
        state: Option<String>,
        street: Option<String>,
        street2: Option<String>,
        zip: Option<String>,
    ) -> Self {
        Self {
            email,
            city,
            country,
            fax,
            internet,
            name,
            mobile,
            phone,
            state,
            street,
            street2,
            zip,
        }
    }
}

impl From<PyContact> for ::lenex::Contact {
    fn from(value: PyContact) -> Self {
        Self {
            email: value.email,
            city: value.city,
            country: value.country,
            fax: value.fax,
            internet: value.internet,
            name: value.name,
            mobile: value.mobile,
            phone: value.phone,
            state: value.state,
            street: value.street,
            street2: value.street2,
            zip: value.zip,
        }
    }
}

impl From<::lenex::Contact> for PyContact {
    fn from(value: ::lenex::Contact) -> Self {
        Self {
            email: value.email,
            city: value.city,
            country: value.country,
            fax: value.fax,
            internet: value.internet,
            name: value.name,
            mobile: value.mobile,
            phone: value.phone,
            state: value.state,
            street: value.street,
            street2: value.street2,
            zip: value.zip,
        }
    }
}
