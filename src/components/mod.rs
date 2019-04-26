//! A collection of components.

pub mod animation;
pub mod camera;
pub mod check_collision;
pub mod collision;
pub mod decrease_velocity;
pub mod gravity;
pub mod inner_size;
pub mod max_velocity;
pub mod parallax;
pub mod push;
pub mod pushable;
pub mod scale_once;
pub mod size;
pub mod solid;
pub mod velocity;

pub mod helpers;

pub mod prelude {
    pub use amethyst::core::transform::Transform;

    pub use super::camera;
    pub use super::collision;
    pub use super::Animation;
    pub use super::Camera;
    pub use super::CheckCollision;
    pub use super::Collision;
    pub use super::DecreaseVelocity;
    pub use super::Gravity;
    pub use super::InnerSize;
    pub use super::MaxVelocity;
    pub use super::Parallax;
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

    pub use crate::geo::Vector;
}

pub use animation::Animation;
pub use camera::Camera;
pub use check_collision::CheckCollision;
pub use collision::Collision;
pub use decrease_velocity::DecreaseVelocity;
pub use gravity::Gravity;
pub use inner_size::InnerSize;
pub use max_velocity::MaxVelocity;
pub use parallax::Parallax;
pub use push::Push;
pub use pushable::Pushable;
pub use scale_once::ScaleOnce;
pub use size::Size;
pub use solid::Solid;
pub use velocity::Velocity;

pub use helpers::prelude::*;
