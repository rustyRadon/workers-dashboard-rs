use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum PersonError {
    #[error("member not found")]
    PersonNotFound,
    #[error("failed to update member")]
    PersonUpdateFailure,
    #[error("failed to create member")]
    PersonCreationFailure,
    #[error("failed to delete member")]
    PersonDeleteFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage(pub String);

impl ErrorMessage {
    pub fn create(person_error: PersonError) -> String {
        match person_error {
            PersonError::PersonNotFound => "member not found".to_string(),
            PersonError::PersonUpdateFailure => "failed to update member".to_string(),
            PersonError::PersonCreationFailure => "failed to create member".to_string(),
            PersonError::PersonDeleteFailure => "failed to delete member".to_string(),
        }
    }
}