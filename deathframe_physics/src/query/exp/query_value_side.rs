use super::{CollisionInnerSideX, CollisionInnerSideY, CollisionSide};
use std::cmp::PartialEq;
use std::hash::Hash;

#[derive(PartialEq, Eq, Clone, Hash, Deserialize)]
pub enum QueryValueSide {
    Left,
    Right,
    Top,
    Bottom,
    Inner,
    InnerSide {
        x: Option<QueryValueInnerSideX>,
        y: Option<QueryValueInnerSideY>,
    },
}

#[derive(PartialEq, Eq, Clone, Hash, Deserialize)]
pub enum QueryValueInnerSideX {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Hash, Deserialize)]
pub enum QueryValueInnerSideY {
    Top,
    Bottom,
}

impl PartialEq<CollisionSide> for QueryValueSide {
    fn eq(&self, other: &CollisionSide) -> bool {
        use CollisionSide as Side;
        match (self, other) {
            (Self::Left, Side::Left) => true,
            (Self::Right, Side::Right) => true,
            (Self::Top, Side::Top) => true,
            (Self::Bottom, Side::Bottom) => true,
            (Self::Inner, Side::Inner { .. }) => true,
            (
                Self::InnerSide {
                    x: self_inner_x,
                    y: self_inner_y,
                },
                Side::Inner {
                    x: other_inner_x,
                    y: other_inner_y,
                },
            ) => {
                (match (self_inner_x, other_inner_x) {
                    (None, _) => true,
                    (Some(self_x), Some(other_x)) => self_x == other_x,
                    (Some(_), None) => false,
                }) && (match (self_inner_y, other_inner_y) {
                    (None, _) => true,
                    (Some(self_y), Some(other_y)) => self_y == other_y,
                    (Some(_), None) => false,
                })
            }
            (_, _) => false,
        }
    }
}

impl PartialEq<CollisionInnerSideX> for QueryValueInnerSideX {
    fn eq(&self, other: &CollisionInnerSideX) -> bool {
        use CollisionInnerSideX as InnerX;
        match (self, other) {
            (Self::Left, InnerX::Left) => true,
            (Self::Right, InnerX::Right) => true,
            (_, _) => false,
        }
    }
}

impl PartialEq<CollisionInnerSideY> for QueryValueInnerSideY {
    fn eq(&self, other: &CollisionInnerSideY) -> bool {
        use CollisionInnerSideY as InnerY;
        match (self, other) {
            (Self::Top, InnerY::Top) => true,
            (Self::Bottom, InnerY::Bottom) => true,
            (_, _) => false,
        }
    }
}
