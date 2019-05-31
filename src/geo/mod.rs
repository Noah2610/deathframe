//! This module includes some miscellaneous stuff related to geography
//! (or algebra, not sure what to call it exactly).

mod anchor;
mod axis;
mod collision;
mod rect;
mod side;
mod vector;

pub mod prelude {
    pub use super::collision::prelude::*;
    pub use super::Anchor;
    pub use super::Axis;
    pub use super::Rect;
    pub use super::RectBuilder;
    pub use super::Side;
    pub use super::Vector;
}

pub use anchor::Anchor;
pub use axis::Axis;
pub use collision::prelude::*;
pub use rect::Rect;
pub use rect::RectBuilder;
pub use side::Side;
pub use vector::Vector;
