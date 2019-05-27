use super::component_prelude::*;

/// Solid entities cannot move into each other;
/// collision detection between solid entities is performed
/// while moving them by their velocities.
/// Solid only affects moving entities and solid entities moving into non-moving, solid entities.
#[derive(Serialize, Deserialize, Default)]
pub struct Solid<T>
where
    T: SolidTag,
{
    pub tag: T,
}

impl<T> Solid<T>
where
    T: SolidTag,
{
    pub fn new(tag: T) -> Self {
        Self { tag }
    }
}

impl<T> Component for Solid<T>
where
    T: 'static + SolidTag,
{
    type Storage = VecStorage<Self>;
}

/// Solid entities' solid tags must implement this trait.
/// The generic solid tag type, that implements this trait,
/// has to define the method `collides_with`.
pub trait SolidTag: Send + Sync + Default + Clone {
    /// This method is passed the other solid tag of the same type as `Self`.
    /// It needs to return `true` if these tags should collide with each other
    /// and `false` if not.
    fn collides_with(&self, other: &Self) -> bool;
}

/// Implement `SolidTag` for all types which also implement `PartialEq`.
impl<T> SolidTag for T
where
    T: Send + Sync + Default + Clone + PartialEq,
{
    fn collides_with(&self, other: &Self) -> bool {
        self == other
    }
}
