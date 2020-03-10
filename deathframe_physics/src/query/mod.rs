pub mod exp;

use crate::collision::prelude::*;
use crate::collision::tag::CollisionTag;
use crate::components::prelude::Collider;
use exp::QueryExpression as QExp;

/// The `Query` can be used to check for collisions
/// on a `Collider`.
pub struct Query<'a, C>
where
    C: 'static + CollisionTag,
{
    collider:           &'a Collider<C>,
    find_expressions:   Vec<QExp<C>>,
    filter_expressions: Vec<QExp<C>>,
}

impl<'a, C> Query<'a, C>
where
    C: 'static + CollisionTag,
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
    pub fn find(mut self, exp: QExp<C>) -> Self {
        self.find_expressions.push(exp);
        self
    }

    /// Checks and adds _all_ collisions that match the given `QExp`.
    pub fn filter(mut self, exp: QExp<C>) -> Self {
        self.filter_expressions.push(exp);
        self
    }

    fn run_expression_on(
        &self,
        exp: &QExp<C>,
        collision: &CollisionData<C>,
    ) -> bool {
        match exp {
            QExp::And(exps) => exps
                .into_iter()
                .all(|e| self.run_expression_on(e, collision)),

            QExp::Or(exps) => exps
                .into_iter()
                .any(|e| self.run_expression_on(e, collision)),

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

    /// Run the query.
    /// Runs both _any_ and _all_ queries (if both exist)
    /// and returns all collisions that match the queries.
    pub fn run(mut self) -> Vec<&'a CollisionData<C>> {
        let query_find = |query: &Self| -> Vec<&'a CollisionData<C>> {
            query
                .find_expressions
                .iter()
                .filter_map(|exp| {
                    query.collider.collisions.values().find(move |collision| {
                        query.run_expression_on(exp, collision)
                    })
                })
                .collect()
        };
        let query_filter = |query: &Self| -> Vec<&'a CollisionData<C>> {
            query
                .filter_expressions
                .iter()
                .map(|exp| {
                    query.collider.collisions.values().filter(
                        move |collision| {
                            query.run_expression_on(exp, collision)
                        },
                    )
                })
                .flatten()
                .collect()
        };

        [query_find(&mut self), query_filter(&mut self)].concat()
    }
}
