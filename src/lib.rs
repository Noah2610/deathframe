#[cfg(feature = "animation")]
pub extern crate animation;
#[cfg(feature = "audio")]
pub extern crate audio;
pub extern crate core;
#[cfg(feature = "physics")]
pub extern crate physics;

pub mod bundles;
pub mod components;
pub mod resources;
pub mod states;
pub mod systems;

pub use core::amethyst;
