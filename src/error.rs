use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML error: {0}")]
    Xml(String),

    #[error("archive error: {0}")]
    Archive(String),

    #[error("unsupported file extension: {0:?}")]
    UnknownExtension(Option<String>),
}
