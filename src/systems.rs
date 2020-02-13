//! Re-exports systems from enabled deathframe crates.

pub mod prelude {
    #[cfg(feature = "animation")]
    pub use animation::systems::prelude::*;
    pub use core::systems::prelude::*;
    #[cfg(feature = "physics")]
    pub use physics::systems::prelude::*;
}

pub use core::systems::system_prelude;
