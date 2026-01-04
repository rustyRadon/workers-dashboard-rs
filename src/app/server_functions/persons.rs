#![allow(unused_imports)]
use crate::app::models::person::{Person, AddPersonRequest, EditPersonRequest, DeletePersonRequest};
use crate::app::errors::{ErrorMessage, PersonError}; 
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[server(GetPersons, "/api")]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError> {
    Ok(retrieve_all_persons().await)
}

#[server(AddPerson, "/api")]
pub async fn add_person(add_person_request: AddPersonRequest) -> Result<Person, ServerFnError> {
    match add_new_person(
        add_person_request.name,
        add_person_request.title,
        add_person_request.level,
        add_person_request.compensation,
    ).await {
        Some(p) => Ok(p),
        None => Err(ServerFnError::new("Database failure during person creation")),
    }
}

#[server(DeletePerson, "/api")]
pub async fn delete_person(delete_person_request: DeletePersonRequest) -> Result<Person, ServerFnError> {
    match delete_team_person(delete_person_request.uuid).await {
        Ok(Some(p)) => Ok(p),
        _ => Err(ServerFnError::new("Delete failed")),
    }
}

#[server(EditPerson, "/api")]
pub async fn edit_person(edit_person_request: EditPersonRequest) -> Result<Person, ServerFnError> {
    match edit_team_person(edit_person_request.uuid, edit_person_request.title, edit_person_request.level, edit_person_request.compensation).await {
        Ok(Some(p)) => Ok(p),
        _ => Err(ServerFnError::new("Edit failed")),
    }
}

#[cfg(feature = "ssr")]
pub async fn retrieve_all_persons() -> Vec<Person> {
    use crate::app::db::database;
    database::get_all_persons().await.unwrap_or_default()
}

#[cfg(feature = "ssr")]
pub async fn add_new_person(name: String, title: String, level: String, compensation: i32) -> Option<Person> {
    use crate::app::db::database;
    use chrono::Local;
    use uuid::Uuid;
    let new_person = Person::new(Uuid::new_v4().to_string(), name, title, level, compensation, Local::now().to_string());
    database::add_person(new_person).await
}

#[cfg(feature = "ssr")]
pub async fn delete_team_person(uuid: String) -> Result<Option<Person>, PersonError> {
    use crate::app::db::database;
    database::delete_person(uuid).await
}

#[cfg(feature = "ssr")]
pub async fn edit_team_person(uuid: String, title: String, level: String, compensation: i32) -> Result<Option<Person>, PersonError> {
    use crate::app::db::database;
    database::update_person(uuid, title, level, compensation).await
}

#[cfg(not(feature = "ssr"))]
pub async fn retrieve_all_persons() -> Vec<Person> { vec![] }
#[cfg(not(feature = "ssr"))]
pub async fn add_new_person(_: String, _: String, _: String, _: i32) -> Option<Person> { None }
#[cfg(not(feature = "ssr"))]
pub async fn delete_team_person(_: String) -> Result<Option<Person>, PersonError> { Err(PersonError::PersonDeleteFailure) }
#[cfg(not(feature = "ssr"))]
pub async fn edit_team_person(_: String, _: String, _: String, _: i32) -> Result<Option<Person>, PersonError> { Err(PersonError::PersonUpdateFailure) }