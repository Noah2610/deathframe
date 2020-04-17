use std::convert::TryFrom;

#[derive(Clone, PartialEq, Debug)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
    Inner {
        x: Option<CollisionInnerSideX>,
        y: Option<CollisionInnerSideY>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum CollisionInnerSideX {
    Left,
    Right,
}

impl TryFrom<CollisionSide> for CollisionInnerSideX {
    type Error = String;
    fn try_from(side: CollisionSide) -> Result<Self, Self::Error> {
        match side {
            CollisionSide::Left => Ok(Self::Left),
            CollisionSide::Right => Ok(Self::Right),
            side => Err(format!(
                "Cannot convert `{:?}` into `CollisionInnerSideX`",
                side
            )),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CollisionInnerSideY {
    Top,
    Bottom,
}

impl TryFrom<CollisionSide> for CollisionInnerSideY {
    type Error = String;
    fn try_from(side: CollisionSide) -> Result<Self, Self::Error> {
        match side {
            CollisionSide::Top => Ok(Self::Top),
            CollisionSide::Bottom => Ok(Self::Bottom),
            side => Err(format!(
                "Cannot convert `{:?}` into `CollisionInnerSideY`",
                side
            )),
        }
    }
}
