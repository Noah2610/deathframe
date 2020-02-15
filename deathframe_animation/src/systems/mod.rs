pub mod prelude {
    pub use super::play_animations::PlayAnimationsSystem;
    pub use super::switch_animations::SwitchAnimationsSystem;
}

mod system_prelude {
    pub(super) use crate::components::prelude::*;
    pub(super) use core::systems::system_prelude::*;
}

mod play_animations;
mod switch_animations;
