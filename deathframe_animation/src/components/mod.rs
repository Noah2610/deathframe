pub mod prelude {
    pub use super::animation::prelude::*;
}

mod component_prelude {
    pub(super) use core::components::component_prelude::*;
}

mod animation;
