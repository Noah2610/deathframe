#[cfg(feature = "deserialize")]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_builder;
extern crate specs;
extern crate specs_derive;

pub mod collision;
pub mod components;
pub mod query;
pub mod systems;

pub use collision::tag::CollisionTag;
