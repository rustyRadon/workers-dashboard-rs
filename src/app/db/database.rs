cfg_if! {
    if #[cfg(feature = "ssr")] {
        use dotenvy::dotenv;
        use std::env;
        use crate::app::models::Person;
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
        static CONNECTION_INITIALIZED: Lazy<std::sync::OnceLock<()>> = Lazy::new(|| std::sync::OnceLock::new());

        fn env_var(key: &str) -> Result<String, Error> {
            env::var(key).map_err(|_| Error::Db(format!("Missing env var: {}", key)))
        }

        pub async fn open_db_connection() -> Result<(), Error> {
            CONNECTION_INITIALIZED.get_or_init(|| {
                dotenv().ok();
            });
            
            if DB.status() == surrealdb::engine::remote::ws::Status::Connected {
                return Ok(());
            }

            let url = env_var("SURREAL_URL")?;
            let user = env_var("SURREAL_USER")?;
            let pass = env_var("SURREAL_PASS")?;
            let ns = env_var("SURREAL_NS")?;
            let db = env_var("SURREAL_DB")?;

            DB.connect::<Ws>(&url).await?;

            DB.signin(Root {
                username: &user,
                password: &pass,
            })
            .await?;

            DB.use_ns(ns).use_db(db).await?;
            Ok(())
        }

        pub async fn get_all_persons() -> Option<Vec<Person>> {
            if let Err(_) = open_db_connection().await {
                return None;
            }
            
            let result = DB.query("SELECT * FROM person ORDER BY joined_date DESC")
                .await;
            
            match result {
                Ok(mut response) => {
                    let persons: Result<Vec<Person>, _> = response.take(0);
                    match persons {
                        Ok(found_persons) => Some(found_persons),
                        Err(e) => {
                            eprintln!("Error taking persons: {:?}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Query error: {:?}", e);
                    None
                }
            }
        }

        pub async fn add_person(new_person: Person) -> Option<Person> {
            if open_db_connection().await.is_err() {
                return None;
            }

            let result = DB
                .create(("person", new_person.uuid.clone()))
                .content(new_person)
                .await;

            match result {
                Ok(created_person) => created_person, 
                Err(e) => {
                    eprintln!("Error creating person: {:?}", e);
                    None
                }
            }
        }
    }
}