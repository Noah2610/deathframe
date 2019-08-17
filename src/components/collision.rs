use std::collections::HashMap;

use amethyst::ecs::world::Index;

use super::component_prelude::*;
use crate::geo::Side;

/// The different states of collision.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum State {
    /// `Enter` means, this collision has _just_ occured in the previous frame.
    Enter,
    /// `SideEnter` means, this collision has existed previously, but the side has changed.
    SideEnter,
    /// `Leave` means, these entities are _no longer in collision_, since the previous frame.
    /// This collision entry will be removed in the next frame.
    Leave,
    /// `Steady` means, these entities have already been in collision in the previous frame,
    /// and still are.
    Steady,
    /// `None` is a marker, which means this collision should be removed.
    /// You should never be able to have a collision in this state, as it is removed
    /// in the same frame as it is set to this state.
    None,
}

impl State {
    /// Returns `true` when state is `Enter`.
    pub fn is_entering(&self) -> bool {
        self == &State::Enter
    }

    /// Returns `true` when state is `Leaving`.
    pub fn is_leaving(&self) -> bool {
        self == &State::Leave
    }

    /// Returns `true` when state is `Steady`.
    pub fn is_steady(&self) -> bool {
        self == &State::Steady
    }
}

/// Collision data. Holds information on in which _state_ of collision these entities are,
/// and from which _side_ they have collided. It does _not_ know which entities are colliding.
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub side:                 Side,
    pub state:                State,
    set_collision_this_frame: bool,
}

impl Data {
    fn should_remove(&self) -> bool {
        if let State::None = self.state {
            true
        } else {
            false
        }
    }

    /// Set state of _NOT_ colliding entity to `Leave` if it was previously
    /// in collision and not `Leave`, otherwise remove the entity from the HashMap.
    #[rustfmt::skip]
    pub fn unset(&mut self) {
        match self.state {
            State::Leave => self.state = State::None, // Stage for removal
            _            => self.state = State::Leave,
        }
    }
}

/// Entities with `CheckCollision` perform collision detection against
/// all other entities with `Collision`, every frame.
/// Depending on if they are in collision, data will be set.
#[derive(Debug, Serialize, Deserialize)]
pub struct Collision {
    pub(crate) collisions: HashMap<Index, Data>,
}

impl Collision {
    pub fn new() -> Self {
        Self {
            collisions: HashMap::new(),
        }
    }

    /// Returns `true` if in collision with _any_ other entity.
    /// NOTE: I'm pretty sure this will also return `true` for leaving collisions (`State::Leave`).
    ///       Should probably return `false` if all collisions are leaving.
    pub fn in_collision(&self) -> bool {
        !self.collisions.is_empty()
    }

    /// Returns `Some` with the collision data, if this entity is in collision with another
    /// entity with the given entity ID. Returns `None` when not in collision with given entity ID.
    pub fn collision_with(&self, entity_id: Index) -> Option<&Data> {
        self.collisions.get(&entity_id)
    }

    /// Returns `true` if this entity is in collision with the given entity ID,
    /// and the collision is not _leaving_ (`State::Leave`).
    #[rustfmt::skip]
    pub fn in_collision_with(&self, entity_id: Index) -> bool {
        if let Some(data) = self.collisions.get(&entity_id) {
            match data.state {
                State::Leave => false,
                _            => true,
            }
        } else {
            false
        }
    }

    /// Is called when an entity is colliding with this entity.
    pub(crate) fn set_collision_with(&mut self, entity_id: Index, side: Side) {
        if let Some(data) = self.collisions.get_mut(&entity_id) {
            // Set state of colliding entity to ...
            data.state = match data {
                // `Enter` if it was `Leave` previously
                Data {
                    state: State::Leave,
                    side: _,
                    ..
                } => State::Enter,
                // `SideEnter` if the side has changed
                Data {
                    state: _, side: s, ..
                } if s != &side => State::SideEnter,
                // `Steady` if it was any other state previously
                _ => State::Steady,
            };
            data.set_collision_this_frame = true;
            data.side = side;
        } else {
            self.collisions.insert(entity_id, Data {
                side,
                state: State::Enter,
                set_collision_this_frame: true,
            });
        }
    }

    /// Should be called every time data changes.
    /// This is handled by the appropriate system.
    pub(crate) fn update(&mut self) {
        let mut to_remove = Vec::new();
        for (&id, collision) in self.collisions.iter_mut() {
            if collision.set_collision_this_frame {
                // Entity collision data was modified this frame, stage for deletion next frame
                collision.set_collision_this_frame = false;
            } else {
                // Entity collision data was NOT modified this frame, set State to `Leave` or remove
                // self.unset_collision_with_entry((id, collision));
                collision.unset();
                if collision.should_remove() {
                    to_remove.push(id);
                }
            }
        }
        for id in to_remove {
            self.collisions.remove(&id);
        }
    }
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}
