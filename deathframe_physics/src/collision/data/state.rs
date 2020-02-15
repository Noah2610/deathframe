use super::CollisionSide;
use crate::collision::tag::CollisionTag;

/// The state of a collision between a Collider and a Collidable.
#[derive(PartialEq)]
pub enum CollisionState<C>
where
    C: CollisionTag,
{
    /// The collision has just occured this frame,
    /// or the CollisionSide has changed since the previous frame.
    Enter(CollisionStateData<C>),
    /// The collision was the same in the previous frame, with the same side.
    Steady(CollisionStateData<C>),
    /// The collision existed in the previous frame,
    /// but doesn't exist in this frame.  This CollisionState
    /// entry will be removed from the CollisionData in the next frame.
    Leave,
}

#[derive(PartialEq)]
pub struct CollisionStateData<C>
where
    C: CollisionTag,
{
    pub side: CollisionSide,
    pub tag:  C,
}
