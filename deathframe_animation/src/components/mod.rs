pub mod prelude {
    pub use super::animation::Animation;
    pub use super::animations_container::AnimationsContainer;
}

mod component_prelude {
    pub(super) use super::prelude::*;
    pub(super) use crate::data::prelude::*;
    pub(super) use core::components::component_prelude::*;
}

mod animation;
mod animations_container;
