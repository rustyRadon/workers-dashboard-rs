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
    pub reports_to: Option<String>, // Stores the UUID of the manager
}

impl Person {
    pub fn new(
        uuid: String,
        name: String,
        title: String,
        level: String,
        compensation: i32,
        joined_date: String,
        reports_to: Option<String>,
    ) -> Self {
        Self {
            uuid,
            name,
            title,
            level,
            compensation,
            joined_date,
            reports_to,
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
    pub reports_to: Option<String>, 
}

impl AddPersonRequest {
    pub fn new(name: String, title: String, level: String, compensation: i32, reports_to: Option<String>) -> Self {
        Self {
            name,
            title,
            level,
            compensation,
            reports_to,
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
    pub reports_to: Option<String>, // Added so you can move people between managers
}

impl EditPersonRequest {
    pub fn new(uuid: String, title: String, level: String, compensation: i32, reports_to: Option<String>) -> Self {
        Self {
            uuid,
            title,
            level,
            compensation,
            reports_to,
        }
    }
}


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