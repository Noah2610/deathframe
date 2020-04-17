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

#[derive(PartialEq, Eq, Clone, Hash, Deserialize)]
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
    /// _All_ given expressions must be true.
    And(Vec<QueryExpression<C>>),
    /// _Any_ of the given expressions must be true.
    Or(Vec<QueryExpression<C>>),
}

#[derive(PartialEq, Eq, Clone, Hash, Deserialize)]
pub enum QueryValueSide {
    Left,
    Right,
    Top,
    Bottom,
    Inner,
    InnerSide(Box<QueryValueSide>),
}

impl PartialEq<CollisionSide> for QueryValueSide {
    fn eq(&self, other: &CollisionSide) -> bool {
        use CollisionSide as Side;
        match (self, other) {
            (Self::Left, Side::Left) => true,
            (Self::Right, Side::Right) => true,
            (Self::Top, Side::Top) => true,
            (Self::Bottom, Side::Bottom) => true,
            (Self::Inner, Side::Inner(_)) => true,
            (
                Self::InnerSide(self_inner_side),
                Side::Inner(Some(other_inner_side)),
            ) => self_inner_side.as_ref() == other_inner_side.as_ref(),
            (_, _) => false,
        }
    }
}

impl Into<CollisionSide> for &QueryValueSide {
    fn into(self) -> CollisionSide {
        match self {
            QueryValueSide::Left => CollisionSide::Left,
            QueryValueSide::Right => CollisionSide::Right,
            QueryValueSide::Top => CollisionSide::Top,
            QueryValueSide::Bottom => CollisionSide::Bottom,
            QueryValueSide::Inner => CollisionSide::Inner(None),
            QueryValueSide::InnerSide(inner_side) => {
                CollisionSide::Inner(Some(Box::new(inner_side.as_ref().into())))
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Deserialize)]
pub enum QueryValueState {
    Enter,
    EnterSide,
    EnterOrEnterSide,
    Steady,
    Leave,
}

impl PartialEq<CollisionState> for QueryValueState {
    fn eq(&self, coll_state: &CollisionState) -> bool {
        match (self, coll_state) {
            (QueryValueState::Enter, CollisionState::Enter(_)) => true,
            (QueryValueState::EnterSide, CollisionState::EnterSide(_)) => true,
            (QueryValueState::EnterOrEnterSide, CollisionState::Enter(_)) => {
                true
            }
            (
                QueryValueState::EnterOrEnterSide,
                CollisionState::EnterSide(_),
            ) => true,
            (QueryValueState::Steady, CollisionState::Steady(_)) => true,
            (QueryValueState::Leave, CollisionState::Leave) => true,
            (_, _) => false,
        }
    }
}
