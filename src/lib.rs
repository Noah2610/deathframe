pub extern crate core;
#[cfg(feature = "physics")]
pub extern crate physics;

pub mod components {
    pub mod prelude {
        pub use core::components::prelude::*;
        #[cfg(feature = "physics")]
        pub use physics::components::prelude::*;
    }

    pub use core::components::*;
    #[cfg(feature = "physics")]
    pub use physics::components::*;
}

pub mod systems {
    pub mod prelude {
        pub use core::systems::prelude::*;
        #[cfg(feature = "physics")]
        pub use physics::systems::prelude::*;
    }

    pub use core::systems::*;
    #[cfg(feature = "physics")]
    pub use physics::systems::*;
}

pub mod resources {
    pub mod prelude {
        pub use core::resources::prelude::*;
    }

    pub use core::resources::*;
}

pub use core::amethyst;
