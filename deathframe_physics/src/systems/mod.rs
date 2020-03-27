pub mod prelude {
    pub use super::apply_base_friction::ApplyBaseFrictionSystem;
    pub use super::apply_gravity::ApplyGravitySystem;
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
mod move_entities;
mod update_collisions;

pub(crate) mod helpers {
    use super::system_prelude::*;
    use specs::storage::MaskedStorage;
    use specs::Component;
    use std::ops::Deref;

    pub fn gen_collision_grid<C, W, DT>(
        entities: &Entities,
        transforms: &Storage<Transform, DT>,
        hitboxes: &ReadStorage<Hitbox>,
        with_collision_tag_comps: &ReadStorage<W>,
        loadables: &ReadStorage<Loadable>,
        loadeds: &ReadStorage<Loaded>,
        padding_opt: Option<Point>,
    ) -> CollisionGrid<C, ()>
    where
        C: CollisionTag,
        W: WithCollisionTag<C> + Component,
        DT: Deref<Target = MaskedStorage<Transform>>,
    {
        let mut grid = CollisionGrid::<C, ()>::default();

        for (entity, transform, hitbox, collidable) in
            (entities, transforms, hitboxes, with_collision_tag_comps).join()
        {
            if is_entity_loaded(entity, loadables, loadeds) {
                let collision_tag = collidable.collision_tag().clone();

                grid.append(gen_collision_rects(
                    entity,
                    &transform,
                    &hitbox,
                    collision_tag,
                    &padding_opt,
                ));
            }
        }

        grid
    }

    pub fn gen_collision_rects<C>(
        entity: Entity,
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
