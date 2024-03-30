// Declare each model file as a submodule
pub mod user;
pub mod product;

// Optionally re-export structs or enums from your submodules if you want them to be
// directly accessible from the `models` module
pub use self::user::{User, NewUser};
// pub use self::product::{Product, NewProduct};
