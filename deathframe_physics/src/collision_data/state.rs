use super::CollisionSide;

/// The state of a collision between a Collider and a Collidable.
pub enum CollisionState {
    /// The collision has just occured this frame,
    /// or the CollisionSide has changed since the previous frame.
    Enter(CollisionSide),
    /// The collision was the same in the previous frame, with the same side.
    Steady(CollisionSide),
    /// The collision existed in the previous frame,
    /// but doesn't exist in this frame.  This CollisionState
    /// entry will be removed from the CollisionData in the next frame.
    Leave,
}
