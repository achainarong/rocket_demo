use diesel::Queryable;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
