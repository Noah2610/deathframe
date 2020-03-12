pub mod prelude {
    pub use super::FindQuery;
}

use super::query_prelude::*;
use specs::world::Index;

/// The `FindQuery` runs a given `QueryExpression` on all
/// collisions, and returns the first match.
/// Finds a collision matching an expression.
pub struct FindQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    collider:   &'a Collider<C>,
    expression: Option<QueryExpression<C>>,
    filter_ids: Option<Vec<Index>>,
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

    /// If given, only match collisions for entities that have one of the given IDs.
    pub fn filter_ids(mut self, ids: Vec<Index>) -> Self {
        self.filter_ids = Some(ids);
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
            filter_ids,
        } = self;

        let exp = expression?;

        let find_predicate = |collision: &&CollisionData<C>| -> bool {
            does_expression_match_collision(&exp, *collision)
        };

        let found_collision = if let Some(filter_ids) = filter_ids {
            filter_ids
                .iter()
                .filter_map(|id| collider.collisions.get(id))
                .find(find_predicate)
        } else {
            collider.collisions.values().find(find_predicate)
        };

        found_collision
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
            filter_ids: None,
        }
    }
}
