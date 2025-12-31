use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::models::Person;
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() -> Result<(), Error> {
            DB.connect::<Ws>("127.0.0.1:8000").await?;
            DB.signin(Root {
                username: "Radon",
                password: "Hassan@surreal123",
            })
            .await?;
            DB.use_ns("surreal").use_db("person").await?;
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
                        Err(_) => None,
                    }
                }
                Err(_) => None,
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
                Ok(created_person) => created_person, // ✅ already Option<Person>
                Err(_) => None,
            }
        }

    }
}