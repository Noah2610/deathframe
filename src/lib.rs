pub extern crate amethyst;
pub extern crate specs_physics;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub mod components;
pub mod systems;

pub mod custom_game_data;
pub mod geo;
pub mod input_manager;
pub mod menu;
pub mod sprite_sheet_handles;
