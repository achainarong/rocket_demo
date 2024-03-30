use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod user_ops;
pub use user_ops::create_user;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    PgConnetion::establish(&database_url)
        .expect(&fomrat!("Error connecting to {}", database_url));
}
