pub mod prelude {
    pub use super::collidable::Collidable;
    pub use super::collider::Collider;
    pub use super::hitbox::Hitbox;
    pub use super::solid::Solid;
    pub use super::velocity::Velocity;
}

mod component_prelude {
    pub(super) use crate::collision::tag::CollisionTag;
    pub(super) use core::components::component_prelude::*;
}

mod collidable;
mod collider;
mod hitbox;
mod solid;
mod velocity;
