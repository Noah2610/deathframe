pub mod prelude {
    pub use super::animation_frame::{AnimationFrame, AnimationFrameBuilder};
    pub use super::{Animation, AnimationBuilder};
}

mod animation_frame;
#[cfg(test)]
mod tests;

use super::component_prelude::*;
use climer::{Time, Timer};
use prelude::*;

/// Animates an entity with `SpriteRender` frame-by-frame.
/// Iterates through different sprites __in the same spritesheet__.
/// Each sprite has a _duration_, in milliseconds, for how long it will be rendered.
#[derive(Component, Builder)]
#[storage(DenseVecStorage)]
#[builder(pattern = "owned")]
pub struct Animation {
    frames:        Box<dyn Iterator<Item = AnimationFrame> + Send + Sync>,
    #[builder(setter(skip), default)]
    current_frame: Option<AnimationFrame>,
    #[builder(setter(skip), default)]
    timer:         Timer,
}

impl Animation {
    /// Returns an `AnimationBuilder`.
    pub fn builder() -> AnimationBuilder {
        AnimationBuilder::default()
    }

    /// Returns the sprite ID of the current frame of animation,
    /// if there is a current `AnimationFrame`.
    pub fn current_sprite_id(&self) -> Option<usize> {
        self.current_frame.as_ref().map(|frame| frame.sprite_id)
    }

    /// Updates the timer and goes to the next frame, if necessary.
    pub(crate) fn update(&mut self) {
        if self.current_frame.is_some() {
            self.timer.update().expect("Couldn't update timer");
            if self.timer.state.is_finished() {
                self.next_frame();
            }
        } else {
            self.setup_first_frame();
        }
    }

    /// Calls `.next()` on the frames Iterator,
    /// and sets up the timer for the new frame.
    fn next_frame(&mut self) {
        if let Some(next_frame) = self.frames.next() {
            // Setup timer
            self.timer.set_target_time(
                Time::builder().milliseconds(next_frame.duration_ms).build(),
            );
            self.timer.start().expect("Couldn't start timer");
            self.current_frame = Some(next_frame);
        } else {
            // TODO: figure out what to do in this situation.
            eprintln!("NO MORE FRAMES IN ANIMATION!");
        }
    }

    fn setup_first_frame(&mut self) {
        self.next_frame();
        if self.current_frame.is_some() {
            // Call the update method again, now that we _know_
            // that we have a `current_frame` set.
            self.update();
        }
    }
}
