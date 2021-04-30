use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("String not supported for ranges")]
    StringRange,
    #[error("Type {0} not supported as an argument")]
    InvalidType(String),
    #[error("The argument {0} already exists")]
    ArgumentAlreadyExists(String),
}
