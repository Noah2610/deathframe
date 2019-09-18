// TODO
// Refactor these structs.
// Make a trait with common methods (which are most of them),
// and implement that trait for both structs.

mod audio;
mod sprite_sheet;
mod texture;

pub mod prelude {
    pub use super::AudioHandles;
    pub use super::SpriteSheetHandles;
    pub use super::TextureHandles;
}

pub use audio::AudioHandles;
pub use sprite_sheet::SpriteSheetHandles;
pub use texture::TextureHandles;
