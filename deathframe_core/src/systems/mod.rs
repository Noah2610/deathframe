//! A collection of systems, giving the components in this crate functionality.

pub mod prelude {
    pub use super::confine_entities::ConfineEntitiesSystem;
    pub use super::entity_loader::EntityLoaderSystem;
    pub use super::follow::FollowSystem;
    pub use super::input_manager::InputManagerSystem;
    pub use super::print_fps::PrintFpsSystem;
    pub use super::scale_sprites::ScaleSpritesSystem;
    pub use super::update_health::UpdateHealthSystem;
    pub use super::update_lifecycle::UpdateLifecycleSystem;
}

pub mod system_prelude {
    pub type TextureHandle = Handle<Texture>;

    pub use amethyst::assets::{AssetStorage, Handle};
    pub use amethyst::core::timing::Time;
    pub use amethyst::ecs::shred::ResourceId;
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
    pub use amethyst::ecs::{SystemData, World};
    pub use amethyst::input::InputHandler;
    pub use amethyst::renderer::sprite::{SpriteSheet, SpriteSheetHandle};
    pub use amethyst::renderer::{
        Camera as AmethystCamera,
        SpriteRender,
        Texture,
    };

    pub use crate::components::prelude::*;
    pub use crate::geo::prelude::*;
    pub use crate::resources::prelude::*;
}

mod confine_entities;
mod entity_loader;
mod follow;
mod input_manager;
mod print_fps;
mod scale_sprites;
mod update_health;
mod update_lifecycle;
