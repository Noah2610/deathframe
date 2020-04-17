#[derive(Clone, PartialEq)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
    Inner(Option<Box<Self>>),
}
