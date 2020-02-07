pub mod prelude {
    pub use super::collidable::Collidable;
    pub use super::collider::Collider;
    pub use super::solid::Solid;
}

mod component_prelude {
    pub(super) use crate::collision::tag::CollisionTag;
    pub(super) use core::components::component_prelude::*;
}

mod collidable;
mod collider;
mod solid;
