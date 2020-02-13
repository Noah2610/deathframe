pub mod prelude {
    pub use super::animation_frame::{AnimationFrame, AnimationFrameBuilder};
    pub use super::{Animation, AnimationBuilder};
}

mod animation_frame;
#[cfg(test)]
mod tests;

use super::component_prelude::*;
use climer::Timer;
use prelude::*;

/// Animates an entity with `SpriteRender` frame-by-frame.
/// Iterates through different sprites __in the same spritesheet__.
/// Each sprite has a _duration_, in milliseconds, for how long it will be rendered.
#[derive(Component, Builder, Default)]
#[storage(DenseVecStorage)]
#[builder(pattern = "owned", default)]
pub struct Animation {
    pub(crate) frames: Vec<AnimationFrame>,
    pub(crate) timer:  Option<Timer>,
}

impl Animation {
    pub fn builder() -> AnimationBuilder {
        AnimationBuilder::default()
    }
}

impl AnimationBuilder {
    pub fn frame<F>(mut self, frame: F) -> Self
    where
        F: Into<AnimationFrame>,
    {
        self.frames
            .get_or_insert_with(Default::default)
            .push(frame.into());
        self
    }
}
