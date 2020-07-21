use super::component_prelude::*;
use std::cmp;

/// The `Follow` component makes an entity with a `Transform`
/// _follow_ another entity with a `Transform`.
/// Gives this component to an entity, which should _follow_ another entity.
#[derive(PartialEq)]
pub struct Follow {
    pub(crate) to_follow: Entity,
    pub(crate) priority:  i32,
    pub(crate) offset:    (f32, f32),
}

impl Follow {
    /// Creates a new `Follow` component, which should follow the given entity.
    pub fn new(to_follow: Entity) -> Self {
        Self {
            to_follow,
            priority: 0,
            offset: (0.0, 0.0),
        }
    }

    /// Set a custom priority for this follower.
    /// Entities with higher follow priority are moved before
    /// entities with lower priority.
    /// Default priority is `0`.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set a positional offset for this follower.
    /// So this entity follows the followed entity with an offset.
    pub fn with_offset(mut self, offset: (f32, f32)) -> Self {
        self.offset = offset;
        self
    }
}

impl Component for Follow {
    type Storage = VecStorage<Self>;
}

impl cmp::PartialOrd for Follow {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}
