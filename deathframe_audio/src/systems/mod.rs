pub mod prelude {
    pub use super::play_sounds::PlaySoundsSystem;
}

mod system_prelude {
    pub(super) use crate::components::prelude::*;
    pub(super) use crate::resources::prelude::*;
    pub(super) use core::systems::system_prelude::*;
}

mod play_sounds;
