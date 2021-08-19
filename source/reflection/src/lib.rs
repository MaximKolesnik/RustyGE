#![feature(ptr_metadata)]
#![feature(unsize)]
#![feature(box_into_inner)]

#[macro_use]
extern crate simple_error;
extern crate lazy_static;

mod casts;
pub mod database;
pub mod module;
pub mod registration;
pub mod desc;
mod variant;
mod types;

pub use types::*;
pub use variant::*;
