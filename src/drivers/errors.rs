use thiserror::Error;

use crate::use_cases::errors::UseCaseError;

#[derive(Error, Debug)]
pub enum DriverError {
    #[error("Use case error: {0}")]
    UseCase(#[from] UseCaseError),
}
