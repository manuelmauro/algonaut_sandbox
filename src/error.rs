extern crate derive_more;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum SandboxError {
    #[error("Sandbox error: {0}")]
    General(String),
}

impl From<algonaut::error::ServiceError> for SandboxError {
    fn from(error: algonaut::error::ServiceError) -> Self {
        SandboxError::General(error.to_string())
    }
}

impl From<algonaut::transaction::error::TransactionError> for SandboxError {
    fn from(error: algonaut::transaction::error::TransactionError) -> Self {
        SandboxError::General(error.to_string())
    }
}
