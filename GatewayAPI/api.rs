#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate serde;

use rocket::serde::{Deserialize, Serialize};

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use rocket_sync_db_pools::database;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

type Users = Arc<Mutex<HashMap<String, String>>>;

#[post("/login", data = "<user>")]
fn login(users: &rocket::State<Users>, user: Json<User>) -> status::Custom<String> {
    let users = users.lock().unwrap();
    if let Some(password) = users.get(&user.username) {
        if password == &user.password {
            return status::Custom(Status::Ok, "Login successful".to_string());
        }
    }
    status::Custom(Status::Unauthorized, "Invalid username or password".to_string())
}

#[post("/register", data = "<user>")]
fn register(users: &rocket::State<Users>, user: Json<User>) -> status::Custom<String> {
    let mut users = users.lock().unwrap();
    if users.contains_key(&user.username) {
        return status::Custom(Status::BadRequest, "Username already exists".to_string());
    }
    users.insert(user.username.clone(), user.password.clone());
    status::Custom(Status::Ok, "Registration successful".to_string())
}

#[launch]
fn rocket() -> _ {
    let users: Users = Arc::new(Mutex::new(HashMap::new()));

    rocket::build()
        .manage(users)
        .mount("/", routes![login, register])
}
