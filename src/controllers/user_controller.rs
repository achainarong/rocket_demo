use rocket::{post, get, put, delete, serde::json::Json};
use crate::models::user::{User, NewUser, UpdateUser};
use crate::db::user_ops;

#[get("/users")]
pub fn all_users() -> &'static str {
    "All Users"
}

#[get("/users/<id>")]
pub fn get_user(id: String)  -> String {
    format!("User ID: {}", id)
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn create(new_user: Json<NewUser>) -> Json<User> {
    Json(db::user_ops::create_user(new_user.into_innter()));
}

pub fn routes() -> Vec<rocket::Route> {
    routes![all_users, get_user]
}


