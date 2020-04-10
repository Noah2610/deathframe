//! Re-exports components from enabled deathframe crates.

pub mod prelude {
    #[cfg(feature = "animation")]
    pub use animation::components::prelude::*;
    #[cfg(feature = "audio")]
    pub use audio::components::prelude::*;
    pub use core::components::prelude::*;
    #[cfg(feature = "physics")]
    pub use physics::components::prelude::*;
}

pub use core::components::component_prelude;
