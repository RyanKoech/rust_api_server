pub mod rustaceans;
pub mod crates;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

// docker-compose logs -f app
// docker-compose exec app cargo run
// docker-compose exec app curl http://127.0.0.1:8000/crates -H 'Content-type: application/json' -H 'Accept: application/json'  -d '{"rustacean_id": 2, "code" : "foo", "name": "Foo crate", "version" : "0.1", "description": "johndoe@gmail.com"}'
// docker-compose exec app curl http://127.0.0.1:8000/crates
// docker-compose exec app curl http://127.0.0.1:8000/crates -X POST -H 'Accept: application/json' 