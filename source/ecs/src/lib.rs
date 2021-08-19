#![feature(unsize)]
#![feature(box_into_inner)]

#[macro_use]
extern crate reflection_proc;
extern crate containers;
extern crate reflection;
extern crate utils;

mod entity;
mod system;
mod world;
mod component;
mod view;
pub mod registration;

pub use entity::*;
pub use system::*;
pub use world::*;
pub use component::*;
pub use view::*;
