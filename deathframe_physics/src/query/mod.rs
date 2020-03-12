pub mod exp;
pub mod find_query;
pub mod matches;

pub mod prelude {
    pub use super::exp::prelude::*;
    pub use super::find_query::prelude::*;
    pub use super::matches::QueryMatches;
    pub use super::Query;
}

mod query_prelude {
    pub(super) use super::exp::QueryExpression as QExp;
    pub(super) use super::Query;
    pub(super) use crate::collision::prelude::*;
    pub(super) use crate::collision::tag::CollisionTag;
    pub(super) use crate::components::prelude::Collider;

    pub(super) fn does_expression_match_collision<C>(
        exp: &QExp<C>,
        collision: &CollisionData<C>,
    ) -> bool
    where
        C: 'static + CollisionTag,
    {
        match exp {
            QExp::And(exps) => exps
                .into_iter()
                .all(|e| does_expression_match_collision(e, collision)),

            QExp::Or(exps) => exps
                .into_iter()
                .any(|e| does_expression_match_collision(e, collision)),

            QExp::IsSide(target_side_qval) => {
                let target_side: CollisionSide = target_side_qval.into();
                if let Some(side) = collision.side() {
                    target_side == side
                } else {
                    false
                }
            }

            QExp::IsState(target_state_qval) => {
                target_state_qval == collision.state
            }

            QExp::IsTag(target_tag) => target_tag == &collision.tag,
        }
    }
}

use crate::collision::prelude::*;
use crate::collision::tag::CollisionTag;
use crate::components::prelude::Collider;
use exp::QueryExpression as QExp;
use matches::QueryMatches;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Query<'a, C>: From<&'a Collider<C>>
where
    C: 'static + CollisionTag,
{
    type Matches;

    fn run(self) -> Self::Matches;
}

// TODO: Remove
/// The `Query` can be used to check for collisions
/// on a `Collider`.
pub struct _Query<'a, C, NA = (), NB = ()>
where
    C: 'static + CollisionTag,
    NA: Eq + Hash,
    NB: Eq + Hash,
{
    collider:           &'a Collider<C>,
    find_expressions:   HashMap<NA, QExp<C>>,
    filter_expressions: HashMap<NB, QExp<C>>,
}

impl<'a, C, NA, NB> _Query<'a, C, NA, NB>
where
    C: 'static + CollisionTag,
    NA: Eq + Hash,
    NB: Eq + Hash,
{
    /// Returns a new `Query` for the given `Collider`.
    pub fn new(collider: &'a Collider<C>) -> Self {
        Self {
            collider,
            find_expressions: Default::default(),
            filter_expressions: Default::default(),
        }
    }

    /// Checks and adds the _first_ collision that matches the given `QExp`.
    /// Adds the matched collision under the `name` key.
    pub fn find(mut self, name: NA, exp: QExp<C>) -> Self {
        self.find_expressions.insert(name, exp);
        self
    }

    /// Checks and adds _all_ collisions that match the given `QExp`.
    /// Adds the matched collisions under the `name` key.
    pub fn filter(mut self, name: NB, exp: QExp<C>) -> Self {
        self.filter_expressions.insert(name, exp);
        self
    }

    /// Run the query.
    /// Runs both _any_ and _all_ queries (if both exist)
    /// and returns a `QueryMatches` struct, containing hashmaps
    /// for both _find_ and _filter_ queries, with the keys
    /// registered in the `find` and `filter` methods,
    /// where the expressions where added.
    pub fn run(self) -> QueryMatches<'a, C, NA, NB> {
        let Self {
            collider,
            find_expressions,
            filter_expressions,
        } = self;

        let find_collisions = find_expressions
            .into_iter()
            .filter_map(|(exp_name, exp)| {
                collider
                    .collisions
                    .values()
                    .find(|collision| {
                        Self::does_expression_match_collision(&exp, collision)
                    })
                    .map(|collision| (exp_name, collision))
            })
            .collect();

        let filter_collisions = filter_expressions
            .into_iter()
            .filter_map(|(exp_name, exp)| {
                let filtered_collisions: Vec<_> = collider
                    .collisions
                    .values()
                    .filter(|collision| {
                        Self::does_expression_match_collision(&exp, collision)
                    })
                    .collect();
                if !filtered_collisions.is_empty() {
                    Some((exp_name, filtered_collisions))
                } else {
                    None
                }
            })
            .collect();

        QueryMatches {
            find:   find_collisions,
            filter: filter_collisions,
        }
    }

    fn does_expression_match_collision(
        exp: &QExp<C>,
        collision: &CollisionData<C>,
    ) -> bool {
        match exp {
            QExp::And(exps) => exps
                .into_iter()
                .all(|e| Self::does_expression_match_collision(e, collision)),

            QExp::Or(exps) => exps
                .into_iter()
                .any(|e| Self::does_expression_match_collision(e, collision)),

            QExp::IsSide(target_side_qval) => {
                let target_side: CollisionSide = target_side_qval.into();
                if let Some(side) = collision.side() {
                    target_side == side
                } else {
                    false
                }
            }

            QExp::IsState(target_state_qval) => {
                target_state_qval == collision.state
            }

            QExp::IsTag(target_tag) => target_tag == &collision.tag,
        }
    }
}
