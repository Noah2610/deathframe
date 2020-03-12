pub mod prelude {
    pub use super::FindQuery;
}

use super::query_prelude::*;

/// The `FindQuery` runs a given `QueryExpression` on all
/// collisions, and returns the first match.
/// Finds a collision matching an expression.
pub struct FindQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    collider:   &'a Collider<C>,
    expression: Option<QueryExpression<C>>,
}

impl<'a, C> FindQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    /// Use the given `QueryExpression` to match collisions when running the query.
    pub fn exp(mut self, exp: QueryExpression<C>) -> Self {
        self.expression = Some(exp);
        self
    }
}

impl<'a, C> Query<'a, C> for FindQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    type Matches = Option<&'a CollisionData<C>>;

    fn run(self) -> Self::Matches {
        let Self {
            collider,
            expression,
        } = self;

        let exp = expression?;

        let matched_collisions = collider
            .collisions
            .values()
            .find(|collision| does_expression_match_collision(&exp, collision));

        matched_collisions
    }
}

impl<'a, C> From<&'a Collider<C>> for FindQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    fn from(collider: &'a Collider<C>) -> Self {
        Self {
            collider,
            expression: None,
        }
    }
}
