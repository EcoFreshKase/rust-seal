use std::fmt::Display;

pub enum RustSealError {
    CliInvalidArgument(String),
    OqsError(String),
}

impl Display for RustSealError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustSealError::CliInvalidArgument(msg) => write!(f, "Invalid CLI argument: {}", msg),
            RustSealError::OqsError(msg) => write!(f, "OQS error: {}", msg),
        }
    }
}
