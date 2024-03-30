#[get("/products")]
pub fn all_products() -> &'static str {
    "All Prodcuts"
}

#[get("/products/<id>")]
pub fn get_products(id: String) -> String {
    format!("Product ID: {}", id)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![all_products, get_products]
}


