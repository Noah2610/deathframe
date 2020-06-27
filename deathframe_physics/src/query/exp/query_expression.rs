use super::prelude::*;
use super::CollisionTag;

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
    /// Negates the result of the given expression.
    Not(Box<QueryExpression<C>>),
}
