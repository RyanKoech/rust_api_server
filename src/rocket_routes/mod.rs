use std::error::Error;
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{json, Value};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

use crate::models::User;
use crate::repositories::UserRepository;

pub mod authorization;
pub mod rustaceans;
pub mod crates;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);


pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
  rocket::error!("{}", e);
  Custom(Status::InternalServerError, json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARACTERS_LONG
        let session_header = req.headers().get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");
        if let Some(header_value) = session_header {
            let mut cache = req.guard::<Connection<CacheConn>>().await
                .expect("Can not connect to Redis in request guard"); // TODO: Use try_outcome
            let mut db = req.guard::<Connection<DbConn>>().await
                .expect("Can not connect to Postgres in request guard"); // TODO: Use try_outcome

            let result = cache.get::<String, i32>(format!("sessions/{}", header_value[1])).await;
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}


// docker-compose logs -f app
// docker-compose exec app cargo run
// docker-compose exec app curl http://127.0.0.1:8000/crates -H 'Content-type: application/json' -H 'Accept: application/json'  -d '{"rustacean_id": 2, "code" : "foo", "name": "Foo crate", "version" : "0.1", "description": "johndoe@gmail.com"}'
// docker-compose exec app curl http://127.0.0.1:8000/crates
// docker-compose exec app curl http://127.0.0.1:8000/crates -X POST -H 'Accept: application/json' 