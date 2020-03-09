//! A collection of components.

pub mod prelude {
    pub use amethyst::core::transform::Transform;
    pub use amethyst::core::Hidden;
    pub use amethyst::renderer::Transparent;

    pub use super::confined::Confined;
    pub use super::follow::Follow;
    pub use super::loadable::Loadable;
    pub use super::loaded::Loaded;
    pub use super::loader::Loader;
    pub use super::scale_once::ScaleOnce;
    pub use super::size::Size;
}

pub mod component_prelude {
    // NOTE: Quick storage type reference
    // DenseVecStorage: Reduced memory usage for LARGE components.
    // HashMapStorage:  "Best suited for rare components."
    // NullStorage:     Storage without data, used as a simple flag.
    // VecStorage:      Preferable for SMALL components (<= 16 bytes || <= 128 bits). For often used components.
    pub use amethyst::ecs::{
        Component,
        DenseVecStorage,
        Entity,
        HashMapStorage,
        NullStorage,
        Storage,
        VecStorage,
    };

    pub use crate::geo::prelude::*;
}

mod confined;
mod follow;
mod loadable;
mod loaded;
mod loader;
mod scale_once;
mod size;
