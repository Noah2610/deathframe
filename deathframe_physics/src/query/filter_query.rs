pub mod prelude {
    pub use super::FilterQuery;
}

use super::query_prelude::*;
use specs::world::Index;

/// The `FilterQuery` runs a given `QueryExpression` on all
/// collisions, and returns all that match.
/// Filters collisions matching an expression.
pub struct FilterQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    collider:   &'a Collider<C>,
    expression: Option<&'a QueryExpression<C>>,
    filter_ids: Option<Vec<Index>>,
}

impl<'a, C> FilterQuery<'a, C>
where
    C: 'static + CollisionTag,
{
    /// Use the given `QueryExpression` to match collisions when running the query.
    pub fn exp(mut self, exp: &'a QueryExpression<C>) -> Self {
        self.expression = Some(exp);
        self
    }

    /// If given, only match collisions for entities that have one of the given IDs.
    pub fn filter_ids(mut self, ids: Vec<Index>) -> Self {
        self.filter_ids = Some(ids);
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
            filter_ids,
        } = self;

        let exp = if let Some(exp) = expression {
            exp
        } else {
            return Vec::new();
        };

        let filter_predicate = |collision: &&CollisionData<C>| -> bool {
            does_expression_match_collision(exp, *collision)
        };

        let matched_collisions = if let Some(filter_ids) = filter_ids {
            filter_ids
                .iter()
                .filter_map(|id| collider.collisions.get(id))
                .filter(filter_predicate)
                .collect()
        } else {
            collider
                .collisions
                .values()
                .filter(filter_predicate)
                .collect()
        };

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
            filter_ids: None,
        }
    }
}
