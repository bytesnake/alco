use thiserror::Error;
use std::num::{ParseIntError, ParseFloatError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("String not supported for ranges")]
    StringRange,
    #[error("Type {0} not supported as an argument")]
    InvalidType(String),
    #[error("The argument {0} already exists")]
    ArgumentAlreadyExists(String),
    #[error("Could not parse the argument type string {0}")]
    InvalidParamTypeStr(String),
    #[error("Parsing integer failed")]
    ParseInt(#[from] ParseIntError),
    #[error("Parsing float failed")]
    ParseFloat(#[from] ParseFloatError),
}
