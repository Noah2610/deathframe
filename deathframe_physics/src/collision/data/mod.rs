pub mod prelude {
    pub use super::CollisionData;
    pub use super::CollisionSide;
    pub use super::CollisionState;
}

mod side;
mod state;

pub use side::CollisionSide;
pub use state::CollisionState;

use crate::collision::tag::CollisionTag;
use core::amethyst::ecs::world::Index;

pub struct CollisionData<C>
where
    C: CollisionTag,
{
    pub state:                       CollisionState,
    pub tag:                         C,
    pub id:                          Index,
    pub(crate) did_update_collision: bool,
}

impl<C> CollisionData<C>
where
    C: CollisionTag,
{
    pub(crate) fn should_remove(&self) -> bool {
        if let CollisionState::Leave = &self.state {
            !self.did_update_collision
        } else {
            false
        }
    }

    /// Set state of _NOT_ colliding entity to `Leave` if it was previously
    /// in collision and not `Leave`, otherwise remove the entity from the HashMap.
    pub(crate) fn unset(&mut self) {
        match self.state {
            CollisionState::Leave => self.did_update_collision = false, // Stage for removal
            _ => {
                // Keep alive for one more frame (at least) with state Leave.
                self.state = CollisionState::Leave;
                self.did_update_collision = true;
            }
        }
    }

    /// Maybe returns the `CollisionSide` of this collision,
    /// depending on which `CollisionState` it is.
    pub(crate) fn side(&self) -> Option<&CollisionSide> {
        use CollisionState::*;

        match &self.state {
            Enter(side) | EnterSide(side) | Steady(side) => Some(side),
            Leave => None,
        }
    }
}
