pub mod prelude {
    pub use super::play_animations::PlayAnimationsSystem;
}

mod system_prelude {
    pub(super) use core::systems::system_prelude::*;
}

mod play_animations;
