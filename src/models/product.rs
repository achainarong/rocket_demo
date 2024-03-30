use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}


