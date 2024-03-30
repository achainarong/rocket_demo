use diesel::prlude::*;
use crate::models::user::{User, NewUser};
use crate::db::establish_connection;

pub fn create_user(new_user: NewUser) -> User {
    let conn = establish_connection();

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&conn)
        .expect("Error saving new user")
}
