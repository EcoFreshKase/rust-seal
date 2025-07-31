use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RustSealError {
    CliInvalidArgument(String),
    OqsError(String),
    IoError(String),
}

impl Display for RustSealError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustSealError::CliInvalidArgument(msg) => write!(f, "Invalid CLI argument: {}", msg),
            RustSealError::OqsError(msg) => write!(f, "OQS error: {}", msg),
            RustSealError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl Error for RustSealError {}

impl From<std::io::Error> for RustSealError {
    fn from(err: std::io::Error) -> Self {
        RustSealError::IoError(err.to_string())
    }
}

impl Into<RustSealError> for oqs::Error {
    fn into(self) -> RustSealError {
        RustSealError::OqsError(self.to_string())
    }
}
