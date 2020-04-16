pub extern crate amethyst;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde;

pub mod components;
pub mod systems;

pub mod custom_game_data;
pub mod geo;
pub mod menu;
pub mod resources;

mod resource_helper;

pub use resource_helper::resource;
