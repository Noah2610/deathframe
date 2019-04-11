//! This module includes some miscellaneous stuff related to geography
//! (or algebra, not sure what to call it exactly).

mod axis;
mod collision;
mod side;

pub mod prelude {
    pub use super::collision::prelude::*;
    pub use super::Axis;
    pub use super::Side;
    pub use super::Vector;
}

pub use axis::Axis;
pub use collision::prelude::*;
pub use side::Side;

pub type Vector = (f32, f32);
