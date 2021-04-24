use super::component_prelude::*;
use crate::collision::data::prelude::*;
use crate::query::Query;
use core::amethyst::ecs::world::Index;
use std::collections::HashMap;

#[derive(Component, Deserialize, Clone)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields, from = "C")]
pub struct Collider<C>
where
    C: 'static + CollisionTag,
{
    pub tag:        C,
    #[serde(skip, default = "default_collisions_data")]
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

    /// Returns a `Query` type for this collider.
    pub fn query<'a, Q>(&'a self) -> Q
    where
        Q: Query<'a, C>,
    {
        Q::from(&self)
    }

    /// Is called when an entity is colliding with this entity.
    pub(crate) fn set_collision_with(
        &mut self,
        entity_id: Index,
        side: CollisionSide,
        tag: C,
    ) {
        if let Some(data) = self.collisions.get_mut(&entity_id) {
            use CollisionState::*;

            // Set state of colliding entity to ...
            data.state = match &data.state {
                // `Enter` if it was `Leave` previously.
                CollisionState::Leave => Enter(side),

                // Collision existed previously and is still exists.
                // Change to `Steady` if the side didn't change,
                // or to `EnterSide` if the side changed.
                Enter(prev_side) | EnterSide(prev_side) | Steady(prev_side) => {
                    if &side == prev_side {
                        Steady(side)
                    } else {
                        EnterSide(side)
                    }
                }
            };
            data.did_update_collision = true;
        } else {
            self.collisions.insert(entity_id, CollisionData {
                state:                CollisionState::Enter(side),
                tag:                  tag,
                id:                   entity_id,
                did_update_collision: true,
            });
        }
    }

    /// Should be called every time data changes.
    /// This is handled by the appropriate system.
    pub(crate) fn update(&mut self) {
        let mut to_remove = Vec::new();
        for (&id, collision) in self.collisions.iter_mut() {
            if collision.did_update_collision {
                // Entity collision data was modified this frame,
                // stage for possible deletion next frame
                collision.did_update_collision = false;
            } else {
                // Entity collision data was NOT modified this frame,
                // set State to `Leave` or remove
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

impl<C> From<C> for Collider<C>
where
    C: CollisionTag,
{
    fn from(tag: C) -> Self {
        Self {
            tag,
            collisions: default_collisions_data(),
        }
    }
}

fn default_collisions_data<C>() -> HashMap<Index, CollisionData<C>>
where
    C: CollisionTag,
{
    HashMap::new()
}
