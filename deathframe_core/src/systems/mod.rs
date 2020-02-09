//! A collection of systems, giving the components in this crate functionality.

mod animation;
mod follow;
mod input_manager;
mod scale_sprites;

pub mod prelude {
    pub use super::AnimationSystem;
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
    pub use crate::resources::InputManager;

    pub use super::helpers::*;
}

pub use animation::AnimationSystem;
pub use follow::FollowSystem;
pub use input_manager::InputManagerSystem;
pub use scale_sprites::ScaleSpritesSystem;

mod helpers {
    use super::system_prelude::*;

    pub fn is_entity_loaded(
        entity: Entity,
        loadables: &ReadStorage<Loadable>,
        loadeds: &ReadStorage<Loaded>,
    ) -> bool {
        loadables.contains(entity) == loadeds.contains(entity)
    }
}
