use std::error::Error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Value};

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

// docker-compose logs -f app
// docker-compose exec app cargo run
// docker-compose exec app curl http://127.0.0.1:8000/crates -H 'Content-type: application/json' -H 'Accept: application/json'  -d '{"rustacean_id": 2, "code" : "foo", "name": "Foo crate", "version" : "0.1", "description": "johndoe@gmail.com"}'
// docker-compose exec app curl http://127.0.0.1:8000/crates
// docker-compose exec app curl http://127.0.0.1:8000/crates -X POST -H 'Accept: application/json' 