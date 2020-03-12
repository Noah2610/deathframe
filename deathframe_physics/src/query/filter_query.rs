pub mod prelude {
    pub use super::FilterQuery;
}

use super::query_prelude::*;

/// The `FilterQuery` runs a given `QueryExpression` on all
/// collisions, and returns all that match.
/// Filters collisions matching an expression.
pub struct FilterQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    collider:   &'a Collider<C>,
    expression: Option<QueryExpression<C>>,
}

impl<'a, C> FilterQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    /// Use the given `QueryExpression` to match collisions when running the query.
    pub fn exp(mut self, exp: QueryExpression<C>) -> Self {
        self.expression = Some(exp);
        self
    }
}

impl<'a, C> Query<'a, C> for FilterQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    type Matches = Vec<&'a CollisionData<C>>;

    fn run(self) -> Self::Matches {
        let Self {
            collider,
            expression,
        } = self;

        let exp = if let Some(exp) = expression {
            exp
        } else {
            return Vec::new();
        };

        let matched_collisions = collider
            .collisions
            .values()
            .filter(|collision| {
                does_expression_match_collision(&exp, collision)
            })
            .collect();

        matched_collisions
    }
}

impl<'a, C> From<&'a Collider<C>> for FilterQuery<'a, C>
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
