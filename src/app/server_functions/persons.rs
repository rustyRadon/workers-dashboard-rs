#![allow(unused_imports)]
use crate::app::models::person::{Person, AddPersonRequest, EditPersonRequest, DeletePersonRequest};
use crate::app::errors::{ErrorMessage, PersonError}; 
use leptos::prelude::*;

#[server]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError> {
    Ok(retrieve_all_persons().await)
}

#[server]
pub async fn add_person(add_person_request: AddPersonRequest) -> Result<Person, ServerFnError> {
    match add_new_person(
        add_person_request.name,
        add_person_request.title,
        add_person_request.level,
        add_person_request.compensation,
    ).await {
        Some(created_person) => Ok(created_person),
        None => Err(ServerFnError::new("Error in creating person!")),
    }
}

#[server]
pub async fn delete_person(
    delete_person_request: DeletePersonRequest,
) -> Result<Person, ServerFnError> {
    match delete_team_person(delete_person_request.uuid).await {
        Ok(Some(person)) => Ok(person),
        Ok(None) => Err(ServerFnError::new(ErrorMessage::create(PersonError::PersonDeleteFailure))),
        Err(e) => Err(ServerFnError::new(ErrorMessage::create(e))),
    }
}

#[server]
pub async fn edit_person(edit_person_request: EditPersonRequest) -> Result<Person, ServerFnError> {
    match edit_team_person(
        edit_person_request.uuid,
        edit_person_request.title,
        edit_person_request.level,
        edit_person_request.compensation,
    ).await {
        Ok(Some(person)) => Ok(person),
        Ok(None) => Err(ServerFnError::new(ErrorMessage::create(PersonError::PersonUpdateFailure))),
        Err(e) => Err(ServerFnError::new(ErrorMessage::create(e))),
    }
}

// --- SSR Implementation ---
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

    let uuid = Uuid::new_v4().to_string();
    let current_formatted = Local::now().to_string();
    let new_person = Person::new(uuid, name, title, level, compensation, current_formatted);
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
pub async fn delete_team_person(_: String) -> Result<Option<Person>, PersonError> { unimplemented!() }
#[cfg(not(feature = "ssr"))]
pub async fn edit_team_person(_: String, _: String, _: String, _: i32) -> Result<Option<Person>, PersonError> { unimplemented!() }