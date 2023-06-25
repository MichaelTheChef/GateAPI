use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use super::models::User;
use super::Users;

#[post("/login", data = "<user>")]
pub fn login(users: &State<Users>, user: Json<User>) -> status::Custom<String> {
    let users = users.lock().unwrap();
    if let Some(password) = users.get(&user.username) {
        if password == &user.password {
            return status::Custom(Status::Ok, "Login successful".to_string());
        }
    }
    status::Custom(Status::Unauthorized, "Invalid username or password".to_string())
}

#[post("/register", data = "<user>")]
pub fn register(users: &State<Users>, user: Json<User>) -> status::Custom<String> {
    let mut users = users.lock().unwrap();
    if users.contains_key(&user.username) {
        return status::Custom(Status::BadRequest, "Username already exists".to_string());
    }
    users.insert(user.username.clone(), user.password.clone());
    status::Custom(Status::Ok, "Registration successful".to_string())
}
