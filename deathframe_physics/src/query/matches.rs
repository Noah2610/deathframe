use crate::collision::data::CollisionData;
use crate::collision::tag::CollisionTag;
use std::collections::HashMap;
use std::hash::Hash;

/// Created when running the `Query` with the `Query::run` function.
/// Returns all matched collisions, split into `find` and `filter`
/// fields, depending on which expression type was used to match the collision.
pub struct QueryMatches<'a, C, NA, NB>
where
    C: 'static + CollisionTag,
    NA: Eq + Hash,
    NB: Eq + Hash,
{
    /// Single collisions that were matched with a _find_ expression.
    pub find:   HashMap<NA, &'a CollisionData<C>>,
    /// Multiple collisions that were matched with a _filter_ expression.
    pub filter: HashMap<NB, Vec<&'a CollisionData<C>>>,
}
