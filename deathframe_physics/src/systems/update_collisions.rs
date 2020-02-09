//! TODO: Rewrite this...

use super::system_prelude::*;
use std::marker::PhantomData;

const PADDING: f32 = 1.0;

// TODO
fn is_entity_loaded(
    entity: Entity,
    loadables: &ReadStorage<Loadable>,
    loadeds: &ReadStorage<Loaded>,
) -> bool {
    loadables.contains(entity) == loadeds.contains(entity)
}

/// The `UpdateCollisionsSystem` is in charge of setting collision states for colliding entities.
/// Entities with `CheckCollision` (and with `Collision`) check for collision against
/// other entities with `Collision`.
/// Only checks for entities with either NO `Loadable` and NO `Loaded` components
/// or for entities with `Loadable` AND `Loaded` components;
/// does not check for entities with `Loadable` but NOT `Loaded` components.

// NOTE:
// Consider giving `UpdateCollisionsSystem` a `CollisionGrid` field, which stores the generated
// `CollisionGrid` between frames; then only update `CollisionRect`s within the grid, which do not
// move (which do not have a `Velocity`).
// This might improve performance, as the `CollisionGrid` wouldn't be re-generated every frame.
// It would have to re-generate and remove all `CollisionRect`s with moving entities each frame
// though, so benchmarking would be needed to verify that this would be beneficial.
#[derive(Default)]
pub struct UpdateCollisionsSystem<C>
where
    C: CollisionTag,
{
    _collision_tag: PhantomData<C>,
}

impl<C> UpdateCollisionsSystem<C>
where
    C: CollisionTag,
{
    fn get_collision_grid(
        entities: &Entities,
        transforms: &ReadStorage<Transform>,
        hitboxes: &ReadStorage<Hitbox>,
        collidables: &ReadStorage<Collidable<C>>,
        loadables: &ReadStorage<Loadable>,
        loadeds: &ReadStorage<Loaded>,
    ) -> CollisionGrid<C, ()> {
        let mut grid = CollisionGrid::<C, ()>::default();

        for (entity, transform, hitbox, collidable) in
            (entities, transforms, hitboxes, collidables).join()
        {
            if is_entity_loaded(entity, loadables, loadeds) {
                let entity_id = entity.id();
                let entity_pos: Point = {
                    let trans = transform.translation();
                    Point::new(trans.x, trans.y)
                };
                let entity_tag = &collidable.tag;

                let base_collision_rect = CollisionRect::<C, ()>::builder()
                    .id(entity_id)
                    .tag(entity_tag.clone());

                // Create the CollisionRect(s) for this entity.
                // Multiple CollisionRects may exist, because an entity
                // can have multiple Hitboxes (Hitbox parts).
                grid.append(
                    hitbox
                        .rects
                        .iter()
                        .map(|hitbox_rect| {
                            base_collision_rect
                                .clone()
                                .rect(hitbox_rect.offset(&entity_pos))
                                .build()
                        })
                        .collect(),
                );
            }
        }

        grid
    }
}

impl<'a, C> System<'a> for UpdateCollisionsSystem<C>
where
    C: CollisionTag + 'static,
{
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Hitbox>,
        WriteStorage<'a, Collider<C>>,
        ReadStorage<'a, Collidable<C>>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            hitboxes,
            mut colliders,
            collidables,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        let collision_grid = Self::get_collision_grid(
            &entities,
            &transforms,
            &hitboxes,
            &collidables,
            &loadables,
            &loadeds,
        );

        // let collision_grid = CollisionGrid::new(
        //     (
        //         &entities,
        //         &transforms,
        //         sizes.maybe(),
        //         loadables.maybe(),
        //         loadeds.maybe(),
        //         &mut collisions,
        //     )
        //         .join()
        //         .filter_map(
        //             |(
        //                 entity,
        //                 transform,
        //                 size_opt,
        //                 loadable_opt,
        //                 loaded_opt,
        //                 _,
        //             )| {
        //                 if let (None, None) | (Some(_), Some(_)) =
        //                     (loadable_opt, loaded_opt)
        //                 {
        //                     let entity_id = entity.id();
        //                     let pos = transform.translation();
        //                     // Create a CollisionRect with increased size, for touch collision checking
        //                     Some(
        //                         CollisionRectBuilder::default()
        //                             .id(entity_id)
        //                             .with_pos_and_maybe_size(
        //                                 (pos.x - PADDING, pos.y - PADDING)
        //                                     .into(),
        //                                 size_opt.map(|size| {
        //                                     (size.w + PADDING, size.h + PADDING)
        //                                         .into()
        //                                 }),
        //                             )
        //                             .build(),
        //                     )
        //                 } else {
        //                     None
        //                 }
        //             },
        //         )
        //         .collect::<Vec<CollisionRect<(), ()>>>(),
        // );

        // for (entity, collision, _, loadable_opt, loaded_opt) in (
        //     &entities,
        //     &mut collisions,
        //     &check_collisions,
        //     loadables.maybe(),
        //     loadeds.maybe(),
        // )
        //     .join()
        // {
        //     if let (None, None) | (Some(_), Some(_)) =
        //         (loadable_opt, loaded_opt)
        //     {
        //         if let Some(rect) = collision_grid.rect_by_id(entity.id()) {
        //             let colliding = collision_grid.colliding_with(rect);
        //             if !colliding.is_empty() {
        //                 let rect_side_rects =
        //                     create_collision_rects_for_sides_from(rect);
        //                 for other_rect in colliding {
        //                     // Check which side is in collision
        //                     if let Some(side) =
        //                         rect_side_rects.collides_with_side(other_rect)
        //                     {
        //                         collision.set_collision_with(
        //                             other_rect.id.expect(
        //                                 "`CollisionRect` should have an `id` \
        //                                  here",
        //                             ),
        //                             side,
        //                         );
        //                     }
        //                 }
        //             }
        //         }

        //         collision.update();
        //     }
        // }
    }
}

// struct CollisionRectSides {
//     pub inner:  CollisionRect<(), Side>,
//     pub top:    CollisionRect<(), Side>,
//     pub bottom: CollisionRect<(), Side>,
//     pub left:   CollisionRect<(), Side>,
//     pub right:  CollisionRect<(), Side>,
// }

// impl CollisionRectSides {
//     pub fn collides_with_side<C>(
//         &self,
//         target_rect: &CollisionRect<(), C>,
//     ) -> Option<Side> {
//         let expect_msg =
//             "`CollisionRect` (for sides) should have custom data with `Side`";
//         if CollisionGrid::<(), ()>::do_rects_collide(target_rect, &self.inner) {
//             Some(self.inner.custom.expect(expect_msg))
//         } else if CollisionGrid::<(), ()>::do_rects_collide(
//             target_rect,
//             &self.top,
//         ) {
//             Some(self.top.custom.expect(expect_msg))
//         } else if CollisionGrid::<(), ()>::do_rects_collide(
//             target_rect,
//             &self.bottom,
//         ) {
//             Some(self.bottom.custom.expect(expect_msg))
//         } else if CollisionGrid::<(), ()>::do_rects_collide(
//             target_rect,
//             &self.left,
//         ) {
//             Some(self.left.custom.expect(expect_msg))
//         } else if CollisionGrid::<(), ()>::do_rects_collide(
//             target_rect,
//             &self.right,
//         ) {
//             Some(self.right.custom.expect(expect_msg))
//         } else {
//             None
//         }
//     }
// }

// fn create_collision_rects_for_sides_from<C>(
//     coll_rect: &CollisionRect<(), C>,
// ) -> CollisionRectSides {
//     CollisionRectSides {
//         inner:  CollisionRect {
//             id:     coll_rect.id,
//             rect:   Rect {
//                 top:    coll_rect.rect.top - PADDING,
//                 bottom: coll_rect.rect.bottom + PADDING,
//                 left:   coll_rect.rect.left + PADDING,
//                 right:  coll_rect.rect.right - PADDING,
//             },
//             tag:    None,
//             custom: Some(Side::Inner),
//         },
//         top:    CollisionRect {
//             id:     coll_rect.id,
//             rect:   Rect {
//                 top:    coll_rect.rect.top,
//                 bottom: coll_rect.rect.bottom + PADDING,
//                 left:   coll_rect.rect.left + PADDING,
//                 right:  coll_rect.rect.right - PADDING,
//             },
//             tag:    None,
//             custom: Some(Side::Top),
//         },
//         bottom: CollisionRect {
//             id:     coll_rect.id,
//             rect:   Rect {
//                 top:    coll_rect.rect.top - PADDING,
//                 bottom: coll_rect.rect.bottom,
//                 left:   coll_rect.rect.left + PADDING,
//                 right:  coll_rect.rect.right - PADDING,
//             },
//             tag:    None,
//             custom: Some(Side::Bottom),
//         },
//         left:   CollisionRect {
//             id:     coll_rect.id,
//             rect:   Rect {
//                 top:    coll_rect.rect.top - PADDING,
//                 bottom: coll_rect.rect.bottom + PADDING,
//                 left:   coll_rect.rect.left,
//                 right:  coll_rect.rect.right - PADDING,
//             },
//             tag:    None,
//             custom: Some(Side::Left),
//         },
//         right:  CollisionRect {
//             id:     coll_rect.id,
//             rect:   Rect {
//                 top:    coll_rect.rect.top - PADDING,
//                 bottom: coll_rect.rect.bottom + PADDING,
//                 left:   coll_rect.rect.left + PADDING,
//                 right:  coll_rect.rect.right,
//             },
//             tag:    None,
//             custom: Some(Side::Right),
//         },
//     }
// }
