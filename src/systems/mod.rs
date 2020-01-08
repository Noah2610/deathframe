//! A collection of systems, giving the components in this crate functionality.

mod animation;
mod collision;
mod confine_entities;
mod decrease_velocities;
mod follow;
mod gravity;
mod input_manager;
mod limit_velocities;
mod move_entities;
mod parallax;
mod scale_sprites;

pub mod prelude {
    pub use super::AnimationSystem;
    pub use super::CollisionSystem;
    pub use super::ConfineEntitiesSystem;
    pub use super::DecreaseVelocitiesSystem;
    pub use super::FollowSystem;
    pub use super::GravitySystem;
    pub use super::InputManagerSystem;
    pub use super::LimitVelocitiesSystem;
    pub use super::MoveEntitiesSystem;
    pub use super::ParallaxSystem;
    pub use super::ScaleSpritesSystem;
}

pub mod system_prelude {
    pub type TextureHandle = Handle<Texture>;

    pub use amethyst::assets::{AssetStorage, Handle};
    pub use amethyst::core::timing::Time;
    pub use amethyst::ecs::world::Index;
    pub use amethyst::ecs::{
        Entities,
        Entity,
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        Storage,
        System,
        Write,
        WriteExpect,
        WriteStorage,
    };
    pub use amethyst::input::InputHandler;
    pub use amethyst::renderer::sprite::{SpriteSheet, SpriteSheetHandle};
    pub use amethyst::renderer::{
        Camera as AmethystCamera,
        SpriteRender,
        Texture,
    };

    pub use super::helpers::*;
    pub use crate::components::prelude::*;
    pub use crate::geo::prelude::*;
    pub use crate::input_manager::InputManager;
}

mod helpers;

pub use animation::AnimationSystem;
pub use collision::CollisionSystem;
pub use confine_entities::ConfineEntitiesSystem;
pub use decrease_velocities::DecreaseVelocitiesSystem;
pub use follow::FollowSystem;
pub use gravity::GravitySystem;
pub use input_manager::InputManagerSystem;
pub use limit_velocities::LimitVelocitiesSystem;
pub use move_entities::MoveEntitiesSystem;
pub use parallax::ParallaxSystem;
pub use scale_sprites::ScaleSpritesSystem;
