//! A collection of components.

pub mod animation;
pub mod animations_container;
pub mod check_collision;
pub mod follow;
pub mod inner_size;
pub mod loadable;
pub mod loaded;
pub mod scale_once;
pub mod size;

pub mod prelude {
    pub use amethyst::core::transform::Transform;
    pub use amethyst::core::Hidden;
    pub use amethyst::renderer::Transparent;

    pub use super::Animation;
    pub use super::AnimationsContainer;
    pub use super::CheckCollision;
    pub use super::Follow;
    pub use super::InnerSize;
    pub use super::Loadable;
    pub use super::Loaded;
    pub use super::ScaleOnce;
    pub use super::Size;
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

pub use animation::Animation;
pub use animations_container::AnimationsContainer;
pub use check_collision::CheckCollision;
pub use follow::Follow;
pub use inner_size::InnerSize;
pub use loadable::Loadable;
pub use loaded::Loaded;
pub use scale_once::ScaleOnce;
pub use size::Size;
