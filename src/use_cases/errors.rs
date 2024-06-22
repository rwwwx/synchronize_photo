use std::error::Error;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum UseCaseError {
    #[error("Something went wrong with PhotoProvider: {0}.")]
    PhotoProvider(String),
    #[error("Unknown error: {0}.")]
    Unknown(Box<dyn Error>),
}
