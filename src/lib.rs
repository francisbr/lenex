//! Parse and generate lenex documents.

mod error;
mod file;
mod model;

pub use error::{Error, Result};
pub use model::{Constructor, Contact, Lenex};
