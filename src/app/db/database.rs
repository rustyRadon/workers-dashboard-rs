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

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() -> Result<(), Error> {
            dotenv().ok();
            
            let url = env::var("SURREAL_URL").map_err(|e| Error::Db(surrealdb::error::Db::Thrown(e.to_string())))?;
            let user = env::var("SURREAL_USER").map_err(|e| Error::Db(surrealdb::error::Db::Thrown(e.to_string())))?;
            let pass = env::var("SURREAL_PASS").map_err(|e| Error::Db(surrealdb::error::Db::Thrown(e.to_string())))?;
            let ns = env::var("SURREAL_NS").map_err(|e| Error::Db(surrealdb::error::Db::Thrown(e.to_string())))?;
            let db = env::var("SURREAL_DB").map_err(|e| Error::Db(surrealdb::error::Db::Thrown(e.to_string())))?;

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
            let _ = open_db_connection().await;
            let mut response = DB.query("SELECT * FROM person ORDER BY joined_date DESC")
                .await.ok()?;
            response.take::<Vec<Person>>(0).ok()
        }

        pub async fn add_person(new_person: Person) -> Option<Person> {
            let _ = open_db_connection().await;
            DB.create::<Option<Person>>(("person", new_person.uuid.clone()))
                .content(new_person)
                .await
                .ok()
                .flatten()
        }

        pub async fn delete_person(uuid: String) -> Result<Option<Person>, crate::app::errors::PersonError> {
            open_db_connection().await.map_err(|_| crate::app::errors::PersonError::PersonDeleteFailure)?;
            DB.delete::<Option<Person>>(("person", uuid))
                .await
                .map_err(|_| crate::app::errors::PersonError::PersonDeleteFailure)
        }

        pub async fn update_person(uuid: String, title: String, level: String, compensation: i32) -> Result<Option<Person>, crate::app::errors::PersonError> {
            open_db_connection().await.map_err(|_| crate::app::errors::PersonError::PersonUpdateFailure)?;
            
            DB.update::<Option<Person>>(("person", uuid))
                .merge(json!({
                    "title": title,
                    "level": level,
                    "compensation": compensation
                }))
                .await
                .map_err(|_| crate::app::errors::PersonError::PersonUpdateFailure)
        }
    }
}