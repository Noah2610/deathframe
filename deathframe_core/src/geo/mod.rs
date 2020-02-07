//! This module includes some miscellaneous stuff related to geography
//! (or algebra, not sure what to call it exactly).

mod axis;

pub mod prelude {
    pub use super::Axis;
    pub use super::Vector;
}

use amethyst::core::math;

pub type Vector<T = f32> = math::Vector2<T>;
pub type Point<T = f32> = math::Point2<T>;

pub use axis::Axis;
