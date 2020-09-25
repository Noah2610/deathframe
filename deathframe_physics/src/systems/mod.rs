pub mod prelude {
    pub use super::apply_base_friction::ApplyBaseFrictionSystem;
    pub use super::apply_gravity::ApplyGravitySystem;
    pub use super::handle_taking_damage::HandleTakingDamageSystem;
    pub use super::move_entities::MoveEntitiesSystem;
    pub use super::update_collisions::UpdateCollisionsSystem;
}

mod system_prelude {
    pub(crate) use super::helpers::*;
    pub(crate) use crate::collision::prelude::*;
    pub(crate) use crate::collision::tag::CollisionTag;
    pub(crate) use crate::components::helpers::WithCollisionTag;
    pub(crate) use crate::components::prelude::*;
    pub(crate) use core::geo::prelude::*;
    pub(crate) use core::systems::system_prelude::*;
}

mod apply_base_friction;
mod apply_gravity;
mod handle_taking_damage;
mod move_entities;
mod update_collisions;

pub(crate) mod helpers {
    use super::system_prelude::*;
    use core::amethyst::ecs::storage::MaskedStorage;
    use core::amethyst::ecs::Component;
    use std::ops::Deref;

    pub fn gen_collision_grid<C, W, DT>(
        entities: &Entities,
        transforms: &Storage<Transform, DT>,
        hitboxes: &ReadStorage<Hitbox>,
        with_collision_tag_comps: &ReadStorage<W>,
        unloaded_store: &ReadStorage<Unloaded>,
        padding_opt: Option<Point>,
    ) -> CollisionGrid<Entity, C, ()>
    where
        C: CollisionTag,
        W: Component + WithCollisionTag<C>,
        DT: Deref<Target = MaskedStorage<Transform>>,
    {
        let mut grid = CollisionGrid::<Entity, C, ()>::default();

        for (entity, transform, hitbox, collidable, _) in (
            entities,
            transforms,
            hitboxes,
            with_collision_tag_comps,
            !unloaded_store,
        )
            .join()
        {
            let collision_tag = collidable.collision_tag().clone();
            let rects = gen_collision_rects(
                &entity,
                &transform,
                &hitbox,
                collision_tag,
                &padding_opt,
            );
            grid.insert(entity, rects);
        }

        grid
    }

    pub fn gen_collision_rects<C>(
        entity: &Entity,
        transform: &Transform,
        hitbox: &Hitbox,
        collision_tag: C,
        padding_opt: &Option<Point>,
    ) -> Vec<CollisionRect<C, ()>>
    where
        C: CollisionTag,
    {
        let entity_id = entity.id();
        let entity_pos: Point = {
            let trans = transform.translation();
            Point::new(trans.x, trans.y)
        };

        let base_collision_rect = CollisionRect::<C, ()>::builder()
            .id(entity_id)
            .tag(collision_tag);

        // Create the CollisionRect(s) for this entity.
        // Multiple CollisionRects may exist, because an entity
        // can have multiple Hitboxes (Hitbox parts).
        hitbox
            .rects
            .iter()
            .map(|hitbox_rect| {
                let mut rect = hitbox_rect.clone().with_offset(&entity_pos);
                if let Some(padding) = padding_opt {
                    rect = rect.with_padding(&padding);
                }
                base_collision_rect.clone().rect(rect).build().unwrap()
            })
            .collect()
    }
}
