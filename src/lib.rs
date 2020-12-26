use govee_rs::{Client, API_BASE};

#[macro_use]
extern crate lazy_static;

mod keys;
pub use keys::GOVEE_API_KEY;

mod error;
pub use error::Error;

mod light;
pub use light::Light;


lazy_static! {
    pub static ref CLIENT: Client = Client::new(API_BASE, GOVEE_API_KEY);
}
