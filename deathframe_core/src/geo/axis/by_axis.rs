use super::Axis;

/// Anything implementing the `ByAxis` trait,
/// returns an item through the `by_axis` method, by passing an `&Axis`.
/// This is useful when you have a tuple or similar with two items,
/// where each item represents an axis. You can get the specifc item
/// by indexing with an `Axis`. Here's an example, and what code this would save:
/// ```
/// use deathframe_core::geo::prelude::{Axis, ByAxis};
///
/// // Consider this piece of code, where the size is doubled ...
/// let size = (5.0, 5.0);
/// let mut doubled_size = (0.0, 0.0);
/// for axis in Axis::iter() {
///     *(&mut doubled_size).by_axis(&axis) = size.by_axis(&axis) * 2.0;
/// }
///
/// assert_eq!(doubled_size, (10.0, 10.0));
///
/// // Compare to this ...
/// let size = (5.0, 5.0);
/// let mut doubled_size = (0.0, 0.0);
/// for axis in Axis::iter() {
///     match &axis {
///         Axis::X => doubled_size.0 = size.0 * 2.0,
///         Axis::Y => doubled_size.1 = size.1 * 2.0,
///     }
/// }
///
/// assert_eq!(doubled_size, (10.0, 10.0));
/// ```
///
/// The above example may be a bit stupid, but I often had a situation like this,
/// where there was `match &axis` everywhere.
///
/// This trait is automatically implemented for shared and mutable references
/// to tuples and arrays with a size of 2.
pub trait ByAxis {
    type Item;
    fn by_axis(self, axis: &Axis) -> Self::Item;
}

impl<'a, T> ByAxis for &'a (T, T) {
    type Item = &'a T;
    fn by_axis(self, axis: &Axis) -> Self::Item {
        match axis {
            Axis::X => &self.0,
            Axis::Y => &self.1,
        }
    }
}

impl<'a, T> ByAxis for &'a mut (T, T) {
    type Item = &'a mut T;
    fn by_axis(self, axis: &Axis) -> Self::Item {
        match axis {
            Axis::X => &mut self.0,
            Axis::Y => &mut self.1,
        }
    }
}

impl<'a, T> ByAxis for &'a [T; 2] {
    type Item = &'a T;
    fn by_axis(self, axis: &Axis) -> Self::Item {
        match axis {
            Axis::X => &self[0],
            Axis::Y => &self[1],
        }
    }
}

impl<'a, T> ByAxis for &'a mut [T; 2] {
    type Item = &'a mut T;
    fn by_axis(self, axis: &Axis) -> Self::Item {
        match axis {
            Axis::X => &mut self[0],
            Axis::Y => &mut self[1],
        }
    }
}
