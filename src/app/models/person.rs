use serde::{Deserialize, Serialize};
use validator::Validate;

/// The Person model used across the database and UI
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Person {
    pub uuid: String,
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "level is required"))]
    pub level: String,
    #[validate(range(min = 2000, max = 99999))]
    pub compensation: i32,
    pub joined_date: String,
}

impl Person {
    pub fn new(
        uuid: String,
        name: String,
        title: String,
        level: String,
        compensation: i32,
        joined_date: String,
    ) -> Self {
        Self {
            uuid,
            name,
            title,
            level,
            compensation,
            joined_date,
        }
    }
}

/// Request structure for creating a new member
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct AddPersonRequest {
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "level is required"))]
    pub level: String,
    #[validate(range(min = 2000, max = 99999))]
    pub compensation: i32,
}

impl AddPersonRequest {
    pub fn new(name: String, title: String, level: String, compensation: i32) -> Self {
        Self {
            name,
            title,
            level,
            compensation,
        }
    }
}

/// Request structure for updating an existing member
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct EditPersonRequest {
    #[validate(length(min = 1, message = "uuid is required"))]
    pub uuid: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "level is required"))]
    pub level: String,
    #[validate(range(min = 2000, max = 99999))]
    pub compensation: i32,
}

impl EditPersonRequest {
    pub fn new(uuid: String, title: String, level: String, compensation: i32) -> Self {
        Self {
            uuid,
            title,
            level,
            compensation,
        }
    }
}

/// Request structure for deleting a member
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DeletePersonRequest {
    #[validate(length(min = 1, message = "uuid is required"))]
    pub uuid: String,
}

impl DeletePersonRequest {
    pub fn new(uuid: String) -> Self {
        Self { uuid }
    }
}