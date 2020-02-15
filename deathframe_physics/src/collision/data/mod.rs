pub mod prelude {
    pub use super::CollisionData;
    pub use super::CollisionSide;
    pub use super::CollisionState;
    pub use super::CollisionStateData;
}

mod side;
mod state;

pub use side::CollisionSide;
pub use state::{CollisionState, CollisionStateData};

use crate::collision::tag::CollisionTag;

pub struct CollisionData<C>
where
    C: CollisionTag,
{
    pub state:                       CollisionState<C>,
    pub(crate) set_state_this_frame: bool,
}

impl<C> CollisionData<C>
where
    C: CollisionTag,
{
    pub(crate) fn should_remove(&self) -> bool {
        if let CollisionState::Leave = self.state {
            !self.set_state_this_frame
        } else {
            false
        }
    }

    /// Set state of _NOT_ colliding entity to `Leave` if it was previously
    /// in collision and not `Leave`, otherwise remove the entity from the HashMap.
    pub(crate) fn unset(&mut self) {
        match self.state {
            CollisionState::Leave => self.set_state_this_frame = false, // Stage for removal
            _ => {
                // Keep alive for one more frame (at least) with state Leave.
                self.state = CollisionState::Leave;
                self.set_state_this_frame = true;
            }
        }
    }
}
