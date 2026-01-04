use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use dotenvy::dotenv;
        use std::env;
        use crate::app::models::person::Person;
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
        static IS_CONNECTED: AtomicBool = AtomicBool::new(false);

        pub async fn open_db_connection() -> Result<(), Error> {
            if IS_CONNECTED.load(Ordering::SeqCst) {
                return Ok(());
            }

            dotenv().ok();
            
            let url = "127.0.0.1:8000"; 
            let user = "Radon";
            let pass = "Hassan@surreal123";
            let ns = "surreal";
            let db = "person";

            match DB.connect::<Ws>(url).await {
                Ok(_) | Err(surrealdb::Error::Api(surrealdb::error::Api::AlreadyConnected)) => {
                    DB.signin(Root { username: user, password: pass }).await?;
                    DB.use_ns(ns).use_db(db).await?;
                    
                    IS_CONNECTED.store(true, Ordering::SeqCst);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Real DB Connection Error: {:?}", e);
                    Err(e)
                }
            }
        }

        pub async fn add_person(new_person: Person) -> Option<Person> {
            if let Err(e) = open_db_connection().await { 
                eprintln!("DB Connection failed in add_person: {:?}", e);
                return None; 
            }

            let id = ("person", new_person.uuid.clone());
            match DB.create::<Option<Person>>(id)
                .content(new_person)
                .await {
                    Ok(Some(p)) => Some(p),
                    Ok(None) => None,
                    Err(e) => {
                        eprintln!(" DB Create Error: {:?}", e);
                        None
                    }
                }
        }
        
        pub async fn get_all_persons() -> Option<Vec<Person>> {
            let _ = open_db_connection().await;
            let res = DB.query("SELECT * FROM person ORDER BY joined_date DESC").await;
            match res {
                Ok(mut response) => response.take::<Vec<Person>>(0).ok(),
                Err(e) => {
                    eprintln!("DB Query Error: {:?}", e);
                    None
                }
            }
        }

        pub async fn delete_person(uuid: String) -> Result<Option<Person>, crate::app::errors::PersonError> {
            let _ = open_db_connection().await;
            DB.delete::<Option<Person>>(("person", uuid))
                .await
                .map_err(|e| {
                    eprintln!(" DB Delete Error: {:?}", e);
                    crate::app::errors::PersonError::PersonDeleteFailure
                })
        }

        pub async fn update_person(uuid: String, title: String, level: String, compensation: i32) -> Result<Option<Person>, crate::app::errors::PersonError> {
            let _ = open_db_connection().await;
            DB.update::<Option<Person>>(("person", uuid))
                .merge(json!({ 
                    "title": title, 
                    "level": level, 
                    "compensation": compensation 
                }))
                .await
                .map_err(|e| {
                    eprintln!(" DB Update Error: {:?}", e);
                    crate::app::errors::PersonError::PersonUpdateFailure
                })
        }
    }
}