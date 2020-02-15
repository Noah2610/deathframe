pub mod prelude {
    pub use super::data::prelude::*;
    pub use super::grid::CollisionGrid;
    pub use super::rect::CollisionRect;
    pub use super::tag::CollisionTag as _;
}

pub mod data;
pub mod grid;
pub mod rect;
pub mod tag;

#[cfg(test)]
mod tests;
