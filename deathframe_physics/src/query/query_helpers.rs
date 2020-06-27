//! Internal helper functions for Query types.

use super::query_prelude::*;
use QueryExpression as QExp;

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

        QExp::Not(exp) => !does_expression_match_collision(exp, collision),

        QExp::IsSide(target_side) => {
            if let Some(side) = collision.side() {
                target_side == side
            } else {
                false
            }
        }

        QExp::IsState(target_state_qval) => {
            target_state_qval == &collision.state
        }

        QExp::IsTag(target_tag) => target_tag == &collision.tag,
    }
}
