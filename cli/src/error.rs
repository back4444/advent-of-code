use std::{fmt, io};

#[derive(Debug)]
pub enum CustomError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Env(std::env::VarError),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Io(err) => write!(f, "I/O Error: {}", err),
            CustomError::Reqwest(err) => write!(f, "Request Error: {}", err),
            CustomError::Env(err) => write!(f, "Environment Variable Error: {}", err),
        }
    }
}

impl std::error::Error for CustomError {}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> Self {
        CustomError::Io(err)
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> Self {
        CustomError::Reqwest(err)
    }
}

impl From<std::env::VarError> for CustomError {
    fn from(err: std::env::VarError) -> Self {
        CustomError::Env(err)
    }
}
