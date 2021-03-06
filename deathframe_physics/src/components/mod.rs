pub mod prelude {
    pub use super::base_friction::BaseFriction;
    pub use super::collidable::Collidable;
    pub use super::collider::Collider;
    pub use super::damage::prelude::*;
    pub use super::gravity::Gravity;
    pub use super::hitbox::Hitbox;
    pub use super::non_precise_movement::NonPreciseMovement;
    pub use super::solid::Solid;
    pub use super::solid_pushable::SolidPushable;
    pub use super::solid_pusher::SolidPusher;
    pub use super::velocity::Velocity;
}

mod component_prelude {
    pub(super) use super::helpers::WithCollisionTag;
    pub(super) use crate::collision::tag::CollisionTag;
    pub(super) use core::components::component_prelude::*;
}

mod base_friction;
mod collidable;
mod collider;
mod damage;
mod gravity;
mod hitbox;
mod non_precise_movement;
mod solid;
mod solid_pushable;
mod solid_pusher;
mod velocity;

pub mod helpers {
    use crate::collision::tag::CollisionTag;

    pub trait WithCollisionTag<C>
    where
        C: CollisionTag,
    {
        fn collision_tag(&self) -> &C;
    }
}
