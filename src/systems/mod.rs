//! A collection of systems, giving the components in this crate functionality.

mod animation;
mod confine_entities;
mod follow;
mod input_manager;
mod scale_sprites;

pub mod prelude {
    pub use super::AnimationSystem;
    pub use super::ConfineEntitiesSystem;
    pub use super::FollowSystem;
    pub use super::InputManagerSystem;
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

    pub use crate::components::prelude::*;
    pub use crate::geo::prelude::*;
    pub use crate::input_manager::InputManager;
}

pub use animation::AnimationSystem;
pub use confine_entities::ConfineEntitiesSystem;
pub use follow::FollowSystem;
pub use input_manager::InputManagerSystem;
pub use scale_sprites::ScaleSpritesSystem;
