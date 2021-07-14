#![feature(ptr_metadata)]
#![feature(unsize)]

#[macro_use]
extern crate simple_error;
extern crate lazy_static;

mod casts;
mod database;
pub mod module;
pub mod registration;
pub mod desc;
mod variant;
mod types;

pub use types::*;
pub use variant::*;
