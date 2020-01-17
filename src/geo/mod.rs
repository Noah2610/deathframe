//! This module includes some miscellaneous stuff related to geography
//! (or algebra, not sure what to call it exactly).

mod axis;
mod side;

pub mod prelude {
    pub use super::Axis;
    pub use super::Side;
    pub use super::Vector;
}

pub type Vector<T = f32> = specs_physics::nphysics::math::Vector<T>;

pub use axis::Axis;
pub use side::Side;
