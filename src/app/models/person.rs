use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, derive, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct person {
    pub uuid : String,
    #[Validate(length(min = 1, message = "name is required"))]
    pub name:String,
    #[Validate(length(min = 1, message = "title is required"))]
    pub title :String,
    #[Validate(length(min = 1, message = "level is required"))]
    pub level :String,
    #[Validate(range(min = 2000, max= 99999))]
    pub compensation: i32,
    pub joined_date:String,
}

impl person {
    pub fn new(
        uuid: String,
        name: String,
        title: String,
        level: String,
        compensation: i32,
        joined_date: String,
    ) -> person {
        uuid,
        name,
        title,
        level,
        compensation,
        joined_date,
    }
}