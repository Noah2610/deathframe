pub mod prelude {
    pub use super::animation::Animation;
    pub use super::animation_frame::AnimationFrame;
    pub use super::animations_container::AnimationsContainer;
}

mod component_prelude {
    pub(super) use super::animation_frames_iter::AnimationFramesIter;
    pub(super) use super::prelude::*;
    pub(super) use core::components::component_prelude::*;
}

mod animation;
mod animation_frame;
mod animation_frames_iter;
mod animations_container;
