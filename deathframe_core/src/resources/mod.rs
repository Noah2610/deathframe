pub mod input_manager;
pub mod sprite_sheet_handles;

pub mod prelude {
    pub use super::InputManager;
    pub use super::SpriteSheetHandles;
}

pub use input_manager::InputManager;
pub use sprite_sheet_handles::SpriteSheetHandles;
