/// Anchor points. Used with some components,
/// where specifying which anchor point is used is necessary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    LeftTop,
    LeftBottom,
    LeftMiddle,
    RightTop,
    RightBottom,
    RightMiddle,
    MiddleTop,
    MiddleBottom,
    Middle,
}
