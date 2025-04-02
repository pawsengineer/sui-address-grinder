use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GrindArgError {
    #[error("Invalid hex string for ends with")]
    InvalidHexStringEndsWith,

    #[error("Invalid hex string for starts with")]
    InvalidHexStringStartsWith,

    #[error("Invalid signature scheme: {0}")]
    InvalidSignatureScheme(String),
}

#[derive(Debug, Error, PartialEq)]
pub enum KeytoolError {
    #[error("Failed to fetch core count")]
    FailedToFetchCoreCount,
}
