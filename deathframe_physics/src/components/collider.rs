use super::component_prelude::*;
use crate::collision::data::prelude::*;
use specs::world::Index;
use std::collections::HashMap;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collider<T>
where
    T: 'static + CollisionTag,
{
    tag:  T,
    data: HashMap<Index, CollisionData>,
}

impl<T> Collider<T>
where
    T: 'static + CollisionTag,
{
    pub fn new(tag: T) -> Self {
        Self {
            tag,
            data: Default::default(),
        }
    }

    /// Is called when an entity is colliding with this entity.
    pub(crate) fn set_collision_with(
        &mut self,
        entity_id: Index,
        side: CollisionSide,
    ) {
        if let Some(data) = self.data.get_mut(&entity_id) {
            // Set state of colliding entity to ...
            data.state = match data.state {
                // `Enter` if it was `Leave` previously
                CollisionState::Leave => CollisionState::Enter(side),
                // `Steady` if it was `Enter` or `Steady` with the same side previously
                CollisionState::Enter(prev_side)
                | CollisionState::Steady(prev_side)
                    if side == prev_side =>
                {
                    CollisionState::Steady(side)
                }
                // `Enter` with new side, if it was `Enter` or `Steady` with a _different_ side previously
                CollisionState::Enter(_) | CollisionState::Steady(_) => {
                    CollisionState::Enter(side)
                }
            };
            data.set_state_this_frame = true;
        } else {
            self.data.insert(entity_id, CollisionData {
                state:                CollisionState::Enter(side),
                set_state_this_frame: true,
            });
        }
    }

    /// Should be called every time data changes.
    /// This is handled by the appropriate system.
    pub(crate) fn update(&mut self) {
        let mut to_remove = Vec::new();
        for (&id, collision) in self.data.iter_mut() {
            if collision.set_state_this_frame {
                // Entity collision data was modified this frame, stage for possible deletion next frame
                collision.set_state_this_frame = false;
            } else {
                // Entity collision data was NOT modified this frame, set State to `Leave` or remove
                collision.unset();
                if collision.should_remove() {
                    to_remove.push(id);
                }
            }
        }
        for id in to_remove {
            self.data.remove(&id);
        }
    }
}
