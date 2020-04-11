pub mod prelude {
    #[cfg(feature = "audio")]
    pub use audio::resources::prelude::*;
    pub use core::resources::prelude::*;
}

pub use prelude::*;
