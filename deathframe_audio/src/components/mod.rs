pub mod prelude {
    pub use super::sound_player::prelude::*;
}

mod component_prelude {
    pub(super) use core::components::component_prelude::*;
}

mod sound_player;
