pub mod prelude {
    pub use super::collidable::Collidable;
    pub use super::collider::Collider;
    pub use super::solid::Solid;
}

mod component_prelude {
    pub(super) use deathframe::component_prelude::*;
}

mod collidable;
mod collider;
mod solid;
