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
    collider:       &'a Collider<C>,
    expression_any: Option<QExp<C>>,
    expression_all: Option<QExp<C>>,
}

impl<'a, C> Query<'a, C>
where
    C: 'static + CollisionTag,
{
    /// Returns a new `Query` for the given `Collider`.
    pub fn new(collider: &'a Collider<C>) -> Self {
        Self {
            collider,
            expression_any: None,
            expression_all: None,
        }
    }

    /// Checks if the given `QExp` matches _any_ collision.
    pub fn any(mut self, exp: QExp<C>) -> Self {
        self.expression_any = Some(exp);
        self
    }

    /// Checks if the given `QExp` matches _all_ collisions.
    pub fn all(mut self, exp: QExp<C>) -> Self {
        self.expression_all = Some(exp);
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
    /// Runs both _any_ and _all_ queries (if both exist) and checks
    /// if both queries return `true`.  A non-existent query returns `true`.
    pub fn run(mut self) -> bool {
        self.expression_any
            .take()
            .map(|exp| {
                self.collider
                    .collisions
                    .values()
                    .any(|collision| self.run_expression_on(&exp, collision))
            })
            .unwrap_or(true)
            && self
                .expression_all
                .take()
                .map(|exp| {
                    self.collider.collisions.values().all(|collision| {
                        self.run_expression_on(&exp, collision)
                    })
                })
                .unwrap_or(true)
    }
}
