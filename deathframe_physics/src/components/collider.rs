use super::component_prelude::*;
use crate::collision::data::prelude::*;
use crate::query::Query;
use specs::world::Index;
use std::collections::HashMap;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collider<C>
where
    C: 'static + CollisionTag,
{
    pub tag:        C,
    pub collisions: HashMap<Index, CollisionData<C>>,
}

impl<C> Collider<C>
where
    C: 'static + CollisionTag,
{
    pub fn new(tag: C) -> Self {
        Self {
            tag,
            collisions: Default::default(),
        }
    }

    pub fn query(&self) -> Query<C> {
        Query::new(&self)
    }

    /// Is called when an entity is colliding with this entity.
    pub(crate) fn set_collision_with(
        &mut self,
        entity_id: Index,
        side: CollisionSide,
        tag: C,
    ) {
        if let Some(data) = self.collisions.get_mut(&entity_id) {
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
            self.collisions.insert(entity_id, CollisionData {
                state:                CollisionState::Enter(side),
                tag:                  tag,
                set_state_this_frame: true,
            });
        }
    }

    /// Should be called every time data changes.
    /// This is handled by the appropriate system.
    pub(crate) fn update(&mut self) {
        let mut to_remove = Vec::new();
        for (&id, collision) in self.collisions.iter_mut() {
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
            self.collisions.remove(&id);
        }
    }
}
