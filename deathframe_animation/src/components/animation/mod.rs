#[cfg(test)]
mod tests;

use super::component_prelude::*;
use climer::{Time, Timer};

/// Animates an entity with `SpriteRender` frame-by-frame.
/// Iterates through different sprites __in the same spritesheet__.
/// Each sprite has a _duration_, in milliseconds, for how long it will be rendered.
#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
#[serde(from = "Vec<(usize, u64)>")]
pub struct Animation {
    frames:        Vec<AnimationFrame>,
    frames_iter:   Option<AnimationFramesIter>,
    current_frame: Option<AnimationFrame>,
    timer:         Timer,
    has_played:    bool,
}

impl Animation {
    /// Play the animation endlessly.
    pub fn play_cycle(&mut self) {
        self.frames_iter = Some(self.frames.clone().into_iter().cycle().into());
        self.current_frame = None;
        self.has_played = true;
    }

    /// Play the animation once.
    pub fn play_once(&mut self) {
        self.frames_iter = Some(self.frames.clone().into_iter().into());
        self.current_frame = None;
        self.has_played = true;
    }

    /// Returns the sprite ID of the current frame of animation,
    /// if there is a current `AnimationFrame`.
    pub fn current_sprite_id(&self) -> Option<usize> {
        self.current_frame.as_ref().map(|frame| frame.sprite_id)
    }

    /// Updates the timer and goes to the next frame, if necessary.
    pub(crate) fn update(&mut self) {
        if self.is_playing() {
            if self.current_frame.is_some() {
                self.timer.update().expect("Couldn't update timer");
                if self.timer.state.is_finished() {
                    self.next_frame();
                }
            } else {
                self.setup_first_frame();
            }
        }
    }

    pub fn has_played_and_is_finished(&self) -> bool {
        self.has_played && !self.is_playing()
    }

    fn is_playing(&self) -> bool {
        self.frames_iter.is_some()
    }

    fn stop_playing(&mut self) {
        self.frames_iter = None;
        self.current_frame = None;
    }

    /// Calls `.next()` on the frames Iterator,
    /// and sets up the timer for the new frame.
    fn next_frame(&mut self) {
        let mut stop_playing = false;

        if let Some(frames_iter) = self.frames_iter.as_mut() {
            if let Some(next_frame) = frames_iter.next() {
                // Setup timer
                self.timer.set_target_time(
                    Time::builder()
                        .milliseconds(next_frame.duration_ms)
                        .build(),
                );
                self.timer.start().expect("Couldn't start timer");
                self.current_frame = Some(next_frame);
            } else {
                stop_playing = true;
            }
        }

        if stop_playing {
            self.stop_playing();
        }
    }

    fn setup_first_frame(&mut self) {
        self.next_frame();
        if self.is_playing() {
            // Call the update method again, now that we _know_
            // that we have a `current_frame` set.
            self.update();
        }
    }
}

impl<A> From<Vec<A>> for Animation
where
    A: Into<AnimationFrame>,
{
    fn from(frames: Vec<A>) -> Self {
        Self {
            frames:        frames.into_iter().map(Into::into).collect(),
            frames_iter:   Default::default(),
            current_frame: Default::default(),
            timer:         Default::default(),
            has_played:    false,
        }
    }
}
