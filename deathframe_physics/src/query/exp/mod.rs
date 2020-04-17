pub mod prelude {
    pub use super::query_expression::QueryExpression;
    pub use super::query_value_side::{
        QueryValueInnerSideX,
        QueryValueInnerSideY,
        QueryValueSide,
    };
    pub use super::query_value_state::QueryValueState;
}

pub mod prelude_variants {
    pub use super::prelude::QueryExpression::*;
    pub use super::prelude::QueryValueInnerSideX as InnerX;
    pub use super::prelude::QueryValueInnerSideY as InnerY;
    pub use super::prelude::QueryValueSide::*;
    pub use super::prelude::QueryValueState::*;
}

mod query_expression;
mod query_value_side;
mod query_value_state;

use crate::collision::data::prelude::*;
use crate::collision::tag::CollisionTag;
