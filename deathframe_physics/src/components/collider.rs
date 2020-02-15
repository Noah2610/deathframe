use super::component_prelude::*;
use crate::collision::data::prelude::*;
use specs::world::Index;
use std::collections::HashMap;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collider<C>
where
    C: 'static + CollisionTag,
{
    pub(crate) tag: C,
    data:           HashMap<Index, CollisionData<C>>,
}

impl<C> Collider<C>
where
    C: 'static + CollisionTag,
{
    pub fn new(tag: C) -> Self {
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
        tag: C,
    ) {
        let state_data = CollisionStateData { side, tag };
        if let Some(data) = self.data.get_mut(&entity_id) {
            // Set state of colliding entity to ...
            data.state = match data.state {
                // `Enter` if it was `Leave` previously
                CollisionState::Leave => CollisionState::Enter(state_data),
                // `Steady` if it was `Enter` or `Steady` with the same side previously
                CollisionState::Enter(CollisionStateData {
                    side: prev_side,
                    tag: _,
                })
                | CollisionState::Steady(CollisionStateData {
                    side: prev_side,
                    tag: _,
                }) if side == prev_side => CollisionState::Steady(state_data),
                // `Enter` with new side, if it was `Enter` or `Steady` with a _different_ side previously
                CollisionState::Enter(_) | CollisionState::Steady(_) => {
                    CollisionState::Enter(state_data)
                }
            };
            data.set_state_this_frame = true;
        } else {
            self.data.insert(entity_id, CollisionData {
                state:                CollisionState::Enter(state_data),
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
