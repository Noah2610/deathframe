pub mod exp;
pub mod filter_query;
pub mod find_query;
pub mod matches;

pub mod prelude {
    pub use super::exp::prelude::*;
    pub use super::filter_query::prelude::*;
    pub use super::find_query::prelude::*;
    pub use super::matches::QueryMatches;
    pub use super::Query;
}

mod query_prelude {
    pub(super) use super::exp::QueryExpression;
    pub(super) use super::query_helpers::*;
    pub(super) use super::Query;
    pub(super) use crate::collision::prelude::*;
    pub(super) use crate::collision::tag::CollisionTag;
    pub(super) use crate::components::prelude::Collider;
}

mod query_helpers;

use crate::collision::tag::CollisionTag;
use crate::components::prelude::Collider;

pub trait Query<'a, C>: From<&'a Collider<C>>
where
    C: 'static + CollisionTag,
{
    /// The type that is returned from `run`.
    type Matches;

    /// Run the query.
    fn run(self) -> Self::Matches;
}
