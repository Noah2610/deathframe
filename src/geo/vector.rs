use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Vector<T = f32>(pub T, pub T)
where
    T: Debug + Clone + Copy + PartialEq + Default;

impl<T> Vector<T>
where
    T: Debug + Clone + Copy + PartialEq + Default,
{
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }
}

impl<T> Default for Vector<T>
where
    T: Debug + Clone + Copy + PartialEq + Default,
{
    fn default() -> Self {
        Self(T::default(), T::default())
    }
}

impl<T> From<(T, T)> for Vector<T>
where
    T: Debug + Clone + Copy + PartialEq + Default,
{
    fn from((x, y): (T, T)) -> Vector<T> {
        Vector(x, y)
    }
}

impl<T> Into<(T, T)> for Vector<T>
where
    T: Debug + Clone + Copy + PartialEq + Default,
{
    fn into(self) -> (T, T) {
        (self.0, self.1)
    }
}
