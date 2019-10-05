//! A collection of components.

pub mod animation;
pub mod animations_container;
pub mod check_collision;
pub mod collision;
pub mod confined;
pub mod decrease_velocity;
pub mod gravity;
pub mod inner_size;
pub mod loadable;
pub mod loaded;
pub mod max_velocity;
pub mod parallax;
pub mod parallax_repeat;
pub mod push;
pub mod pushable;
pub mod scale_once;
pub mod size;
pub mod solid;
pub mod velocity;

pub mod helpers;

pub mod prelude {
    pub use amethyst::core::transform::Transform;
    pub use amethyst::core::Hidden;
    pub use amethyst::renderer::Transparent;

    pub use super::collision;
    pub use super::solid;
    pub use super::Animation;
    pub use super::AnimationsContainer;
    pub use super::CheckCollision;
    pub use super::Collision;
    pub use super::Confined;
    pub use super::DecreaseVelocity;
    pub use super::Gravity;
    pub use super::InnerSize;
    pub use super::Loadable;
    pub use super::Loaded;
    pub use super::MaxVelocity;
    pub use super::Parallax;
    pub use super::ParallaxRepeat;
    pub use super::Push;
    pub use super::Pushable;
    pub use super::ScaleOnce;
    pub use super::Size;
    pub use super::Solid;
    pub use super::Velocity;
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
pub use collision::Collision;
pub use confined::Confined;
pub use decrease_velocity::DecreaseVelocity;
pub use gravity::Gravity;
pub use inner_size::InnerSize;
pub use loadable::Loadable;
pub use loaded::Loaded;
pub use max_velocity::MaxVelocity;
pub use parallax::Parallax;
pub use parallax_repeat::ParallaxRepeat;
pub use push::Push;
pub use pushable::Pushable;
pub use scale_once::ScaleOnce;
pub use size::Size;
pub use solid::Solid;
pub use velocity::Velocity;

pub use helpers::prelude::*;
