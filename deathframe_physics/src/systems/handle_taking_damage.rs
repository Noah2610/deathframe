use super::system_prelude::*;
use crate::query::prelude::*;
use std::collections::HashMap;
use std::marker::PhantomData;

/// This system makes `TakesDamage` entities take damage
/// from `DealsDamage` entities, that it collides with.
/// Dealing damage means _losing health_.
pub struct HandleTakingDamageSystem<C>
where
    C: CollisionTag,
{
    _c: PhantomData<C>,
}

impl<'a, C> System<'a> for HandleTakingDamageSystem<C>
where
    C: 'static + CollisionTag,
{
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, TakesDamage>,
        ReadStorage<'a, DealsDamage>,
        ReadStorage<'a, Collider<C>>,
        WriteStorage<'a, HealthActionQueue>,
    );

    fn run(
        &mut self,
        (
            entities,
            takes_damage_store,
            deals_damage_store,
            collider_store,
            mut health_action_queue_store,
        ): Self::SystemData,
    ) {
        let mut damage_map = HashMap::new();

        for (entity, deals_damage) in (&entities, &deals_damage_store).join() {
            damage_map.insert(entity.id(), deals_damage.damage);
        }

        let damage_dealing_ids: Vec<Index> =
            damage_map.keys().cloned().collect();

        for (_entity, _takes_damage, collider, health_action_queue) in (
            &entities,
            &takes_damage_store,
            &collider_store,
            &mut health_action_queue_store,
        )
            .join()
        {
            let query_exp = {
                use crate::query::exp::prelude_variants::*;
                IsState(Enter)
            };

            let collisions = collider
                .query::<FilterQuery<C>>()
                .filter_ids(&damage_dealing_ids)
                .exp(&query_exp)
                .run();

            for collision in collisions {
                if let Some(damage) = damage_map.get(&collision.id) {
                    health_action_queue.lose(*damage);
                }
            }
        }
    }
}

impl<C> Default for HandleTakingDamageSystem<C>
where
    C: CollisionTag,
{
    fn default() -> Self {
        Self {
            _c: Default::default(),
        }
    }
}
