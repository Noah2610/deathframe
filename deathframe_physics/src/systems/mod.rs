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
    use std::marker::PhantomData;
    use std::ops::Deref;

    pub struct GenCollisionGridData<'a, C, W, DT>
    where
        C: CollisionTag,
        W: WithCollisionTag<C> + Component,
        DT: Deref<Target = MaskedStorage<Transform>>,
    {
        pub(crate) entities:                     &'a Entities<'a>,
        pub(crate) transforms:                   &'a Storage<'a, Transform, DT>,
        pub(crate) hitboxes:                     &'a ReadStorage<'a, Hitbox>,
        pub(crate) with_collision_tag_comps:     &'a ReadStorage<'a, W>,
        pub(crate) loadables:                    &'a ReadStorage<'a, Loadable>,
        pub(crate) loadeds:                      &'a ReadStorage<'a, Loaded>,
        pub(crate) padding:                      Option<Point>,
        pub(crate) collidable_custom_data_store:
            Option<&'a ReadStorage<'a, CollidableCustomData>>,
        pub(crate) _c:                           PhantomData<C>,
    }

    pub fn gen_collision_grid<C, W, DT>(
        GenCollisionGridData {
            entities,
            transforms,
            hitboxes,
            with_collision_tag_comps,
            collidable_custom_data_store,
            loadables,
            loadeds,
            padding,
            _c: _,
        }: GenCollisionGridData<C, W, DT>,
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
                let entity_id = entity.id();
                let entity_pos: Point = {
                    let trans = transform.translation();
                    Point::new(trans.x, trans.y)
                };
                let entity_tag = collidable.collision_tag();

                let base_collision_rect = CollisionRect::<C, ()>::builder()
                    .id(entity_id)
                    .tag(entity_tag.clone());

                grid.append(
                    // Create the CollisionRect(s) for this entity.
                    // Multiple CollisionRects may exist, because an entity
                    // can have multiple Hitboxes (Hitbox parts).
                    hitbox
                        .rects
                        .iter()
                        .map(|hitbox_rect| {
                            let mut rect =
                                hitbox_rect.clone().with_offset(&entity_pos);
                            if let Some(padding) = padding {
                                rect = rect.with_padding(&padding);
                            }
                            base_collision_rect
                                .clone()
                                .rect(rect)
                                .build()
                                .unwrap()
                        })
                        .collect(),
                );
            }
        }

        grid
    }
}
