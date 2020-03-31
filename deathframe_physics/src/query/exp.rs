use crate::collision::data::prelude::*;
use crate::collision::tag::CollisionTag;
use std::cmp::PartialEq;
use std::hash::Hash;

pub mod prelude {
    pub use super::QueryExpression;
    pub use super::QueryValueSide;
    pub use super::QueryValueState;
}

pub mod prelude_variants {
    pub use super::QueryExpression::*;
    pub use super::QueryValueSide::*;
    pub use super::QueryValueState::*;
}

#[derive(PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum QueryExpression<C>
where
    C: CollisionTag,
{
    /// Checks the collision side.
    IsSide(QueryValueSide),
    /// Checks the collision state.
    IsState(QueryValueState),
    /// Checks the given tag for equality with this collider's tag.
    IsTag(C),
    /// Checks if this collider's tag collides with the given tag.
    CollidesWith(C),
    /// Checks if the given tag collides with this collider's tag.
    OtherCollidesWithSelf(C),
    /// _All_ given expressions must be true.
    And(Vec<QueryExpression<C>>),
    /// _Any_ of the given expressions must be true.
    Or(Vec<QueryExpression<C>>),
}

#[derive(PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum QueryValueSide {
    Left,
    Right,
    Top,
    Bottom,
    Inner,
}

impl Into<CollisionSide> for &QueryValueSide {
    fn into(self) -> CollisionSide {
        match self {
            QueryValueSide::Left => CollisionSide::Left,
            QueryValueSide::Right => CollisionSide::Right,
            QueryValueSide::Top => CollisionSide::Top,
            QueryValueSide::Bottom => CollisionSide::Bottom,
            QueryValueSide::Inner => CollisionSide::Inner,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum QueryValueState {
    Enter,
    Steady,
    Leave,
}

impl PartialEq<CollisionState> for &QueryValueState {
    fn eq(&self, coll_state: &CollisionState) -> bool {
        match (self, coll_state) {
            (QueryValueState::Enter, CollisionState::Enter(_)) => true,
            (QueryValueState::Steady, CollisionState::Steady(_)) => true,
            (QueryValueState::Leave, CollisionState::Leave) => true,
            (_, _) => false,
        }
    }
}
