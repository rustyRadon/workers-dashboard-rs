use crate::app::{models::person::Person, models::person::AddPersonRequest };
use leptos::{prelude::ServerFnError, *};
use serde::*;

#[server(GetPerson, "/api")]
pub async fn  get_persons() -> Result<Vec<Person>, ServerFnError>{
    let persons = retrieve_all_persons().await;
    Ok(persons)
}

#[server(AddPerson, "/api")]
pub async fn add_person(add_person_request: AddPersonRequest) -> Result<Person, ServerFnError> {
    let new_person = add_new_person(
        add_person_request.name,
        add_person_request.title,
        add_person_request.level,
        add_person_request.compensation,
    )
    .await;
    
    match new_person {
        Some(created_person) => Ok(created_person),
        None => Err(ServerFnError::Args(String::from(
            "Error in creating person!",
        ))),
    }
}

#[server(Add_person, "/api")]

cfg_if::cfg_if! {

    if #[cfg(feature = "ssr")] {
        use crate::app::db::database;
        use chrono::{DateTime, Local};
        use uuid::Uuid;
        
        pub async fn retrieve_all_persons() -> Vec<Person> {
            let get_all_persons_result = database::get_all_persons().await;
            match get_all_persons_result {
                Some(found_persons) => found_persons,
                None => Vec::new()
            }
        }

        pub async add_new_person<T>(name:  T, title: T, level: T, compensation: i32)
            -> Option<Person> where T: Into<String> {
                let mut buffer = Uuid::encode_buffer();
                let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

                let current_now = Local::now();
                let current_formatted = current_now.to_string();

                let new_person = Person::new(
                    String::from(Uuid),
                    name.into(),
                    title.into(),
                    level.into(),
                    compensation,
                    current_formatted,
                );
                database::add_person(new_person).await
            }
    }
}    