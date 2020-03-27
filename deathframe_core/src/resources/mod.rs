pub mod entity_component_inserter;
pub mod input_manager;
pub mod sprite_sheet_handles;

pub mod prelude {
    pub use super::EntityComponentInserter;
    pub use super::InputManager;
    pub use super::SpriteSheetHandles;
}

pub use entity_component_inserter::EntityComponentInserter;
pub use input_manager::InputManager;
pub use sprite_sheet_handles::SpriteSheetHandles;
