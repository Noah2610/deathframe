use crate::collision::data::prelude::*;
use crate::collision::tag::CollisionTag;
use std::cmp::PartialEq;

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

#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum QueryExpression<C>
where
    C: CollisionTag,
{
    IsSide(QueryValueSide),
    IsState(QueryValueState),
    IsTag(C),
    And(Vec<QueryExpression<C>>),
    Or(Vec<QueryExpression<C>>),
}

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
