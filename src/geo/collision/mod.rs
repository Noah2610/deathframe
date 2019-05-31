//! This `collision` module includes structs used for collision checking.
//! It's actually pretty simple; create a `CollisionGrid`, fill it with `CollisionRect`s,
//! now the `CollisionGrid` can tell you if a given `CollisionRect` is in collision with other
//! `CollisionRect`s, and with which it is colliding with.

mod collision_grid;
mod collision_rect;

pub mod prelude {
    pub use super::CollisionGrid;
    pub use super::CollisionRect;
    pub use super::CollisionRectBuilder;
}

pub use collision_grid::CollisionGrid;
pub use collision_rect::CollisionRect;
pub use collision_rect::CollisionRectBuilder;
