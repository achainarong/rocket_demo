#[macro_use] extern crate rocket;

mod controllers {
    pub mod user_controller;
    pub mod product_controller;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", controllers::user_controller::routes())
        .mount("/", controllers::product_controller::routes())
}

