extern crate core;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde;
extern crate specs;
extern crate specs_derive;

pub mod collision;
pub mod components;
pub mod query;
pub mod systems;

pub use collision::tag::CollisionTag;
