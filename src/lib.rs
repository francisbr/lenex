//! Parse and generate lenex documents.

mod error;
mod file;
mod format;
mod model;

pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::model::{Lenex, constructor::Constructor, constructor::contact::Contact};
}
