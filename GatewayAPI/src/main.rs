#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate serde;

mod models;
mod routes;

use rocket_sync_db_pools::database;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use models::User;
use routes::{login, register};

type Users = Arc<Mutex<HashMap<String, String>>>;

#[launch]
fn rocket() -> _ {
    let users: Users = Arc::new(Mutex::new(HashMap::new()));

    rocket::build()
        .manage(users)
        .mount("/", routes![login, register])
}
