#[cfg(feature = "animation")]
pub extern crate animation;
pub extern crate core;
#[cfg(feature = "physics")]
pub extern crate physics;

pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

pub use core::amethyst;
