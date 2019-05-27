use std::collections::HashMap;

use amethyst::ecs::world::Index;

use super::system_prelude::*;
use crate::geo::{CollisionGrid, CollisionRect, Vector};

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, InnerSize>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            cameras,
            sizes,
            inner_sizes,
            mut transforms,
            mut velocities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        // TODO: REMOVE
        // let player_data_opt =
        //     (&entities, &players, &transforms, (&sizes).maybe())
        //         .join()
        //         .next()
        //         .map(|(entity, player, transform, size_opt)| {
        //             let translation = transform.translation();
        //             (
        //                 entity.id(),
        //                 (translation.x, translation.y),
        //                 if let Some(size) = size_opt {
        //                     (size.w, size.h)
        //                 } else {
        //                     (1.0, 1.0)
        //                 },
        //             )
        //         });

        // Create a HashMap of all following entities for all cameras.
        let following_entities = (&entities, &cameras)
            .join()
            .filter_map(|(entity_camera, camera)| {
                // If camera is following an entity, find it.
                let data_opt = camera.follow.map(|following_id| {
                    let following_data_opt = (&entities, &transforms)
                        .join()
                        .find_map(|(entity, transform)| {
                            if entity.id() == following_id {
                                let pos = transform.translation();
                                Some((entity.id(), (pos.x, pos.y).into(), None)) // NOTE: Might want to add size here too, but currently id doesn't matter
                            } else {
                                None
                            }
                        });
                    following_data_opt.map(|following_data| {
                        (entity_camera.id(), following_data)
                    })
                });
                // Return `Some` in the appropriate data structure,
                // if camera is following an entity and that entity was found.
                if let Some(data) = data_opt {
                    if data.is_some() {
                        data
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<HashMap<Index, (Index, Vector, Option<Vector>)>>();

        // An entry of `following_entities` may look like this:
        // {
        //   0: ( // camera entity id
        //     1,                 // following entity id
        //     (0.0, 0.0)         // following position
        //     Some((10.0, 10.0)) // following size
        //   )
        // }

        for (entity, camera, transform, size, inner_size_opt, velocity_opt) in (
            &entities,
            &cameras,
            &mut transforms,
            &sizes,
            inner_sizes.maybe(),
            (&mut velocities).maybe(),
        )
            .join()
        {
            let camera_id = entity.id();

            if let Some(following) = following_entities.get(&camera_id) {
                // TODO: REMOVE
                // player_pos is at TOP-LEFT of player sprite for some reason... i'm confused...
                // let player_pos = (
                //     player_pos.0 + player_size.0 * 0.5,
                //     player_pos.1 - player_size.1 * 0.5,
                // );

                let following_id = following.0;
                let following_size_opt = following.2;
                let following_pos =
                    if let Some(following_size) = following_size_opt {
                        // following's position is at TOP-LEFT of following's sprite for some reason...
                        // i'm confused...
                        (
                            (following.1).0 + following_size.0 * 0.5,
                            (following.1).1 + following_size.1 * 0.5,
                        )
                            .into()
                    } else {
                        following.1
                    };
                // I don't remember of what this is the center...
                // I'm too confused to figure it out right now.
                // It seems to work, so hey, _if it ain't broke, don't fix it._
                let center: Vector = (
                    following_pos.0 - size.w * 0.5,
                    following_pos.1 - size.h * 0.5,
                )
                    .into();
                let camera_pos = transform.translation();
                let camera_center =
                    (camera_pos.x + size.w * 0.5, camera_pos.y + size.h * 0.5)
                        .into();

                if let Some(inner_size) = inner_size_opt {
                    let following_rect = CollisionRect::<(), ()>::new(
                        following_id,
                        following_pos,
                        None, // Some(player_size)
                              // TODO: Cleanup. I guess we don't need following's size after all?
                    );
                    let camera_rects = CameraCollisionRects::from((
                        camera_id,
                        camera_center,
                        (size.w, size.h).into(),
                        (inner_size.0.w, inner_size.0.h).into(),
                    ));

                    let mut colliding_x = false;
                    let mut colliding_y = false;

                    // Vertical rects (top/bottom)
                    if CollisionGrid::<(), ()>::do_rects_collide(
                        &following_rect,
                        &camera_rects.top,
                    ) {
                        colliding_y = true;
                        transform
                            .set_y((center.1 - inner_size.0.h * 0.5).ceil());
                    } else if CollisionGrid::<(), ()>::do_rects_collide(
                        &following_rect,
                        &camera_rects.bottom,
                    ) {
                        colliding_y = true;
                        transform
                            .set_y((center.1 + inner_size.0.h * 0.5).floor());
                    }
                    // Horizontal rects (left/right)
                    if CollisionGrid::<(), ()>::do_rects_collide(
                        &following_rect,
                        &camera_rects.left,
                    ) {
                        colliding_x = true;
                        transform
                            .set_x((center.0 + inner_size.0.w * 0.5).floor());
                    } else if CollisionGrid::<(), ()>::do_rects_collide(
                        &following_rect,
                        &camera_rects.right,
                    ) {
                        colliding_x = true;
                        transform
                            .set_x((center.0 - inner_size.0.w * 0.5).ceil());
                    }

                    // When not in collision with outer camera rects,
                    // slowly position camera on player.
                    if let Some(velocity) = velocity_opt {
                        if !colliding_x {
                            let dist =
                                (following_pos.0 - camera_center.0).abs();
                            if dist <= camera.deadzone.0 {
                                velocity.x = 0.0;
                            } else if following_pos.0 > camera_center.0 {
                                velocity.x = camera.base_speed.0 * dist * dt;
                            } else if following_pos.0 < camera_center.0 {
                                velocity.x = -camera.base_speed.0 * dist * dt;
                            }
                        } else {
                            velocity.x = 0.0;
                        }
                        if !colliding_y {
                            let dist =
                                (following_pos.1 - camera_center.1).abs();
                            if dist <= camera.deadzone.1 {
                                velocity.y = 0.0;
                            } else if following_pos.1 > camera_center.1 {
                                velocity.y = camera.base_speed.1 * dist * dt;
                            } else if following_pos.1 < camera_center.1 {
                                velocity.y = -camera.base_speed.1 * dist * dt;
                            }
                        } else {
                            velocity.y = 0.0;
                        }
                    }
                } else {
                    if let Some(velocity) = velocity_opt {
                        let dist = (following_pos.0 - camera_center.0).abs();
                        if dist <= camera.deadzone.0 {
                            velocity.x = 0.0;
                        } else if following_pos.0 > camera_center.0 {
                            velocity.x = camera.base_speed.0 * dist * dt;
                        } else if following_pos.0 < camera_center.0 {
                            velocity.x = -camera.base_speed.0 * dist * dt;
                        }
                        let dist = (following_pos.1 - camera_center.1).abs();
                        if dist <= camera.deadzone.1 {
                            velocity.y = 0.0;
                        } else if following_pos.1 > camera_center.1 {
                            velocity.y = camera.base_speed.1 * dist * dt;
                        } else if following_pos.1 < camera_center.1 {
                            velocity.y = -camera.base_speed.1 * dist * dt;
                        }
                    } else {
                        transform.set_x(center.0);
                        transform.set_y(center.1);
                    }
                }
            } else {
                // Camera isn't following an entity.
                // Just don't do anything, I guess?
            }
        }
    }
}

struct CameraCollisionRects {
    pub top:    CollisionRect<(), ()>,
    pub bottom: CollisionRect<(), ()>,
    pub left:   CollisionRect<(), ()>,
    pub right:  CollisionRect<(), ()>,
}

impl From<(Index, Vector, Vector, Vector)> for CameraCollisionRects {
    fn from(
        (id, pos, size, inner_size): (Index, Vector, Vector, Vector),
    ) -> Self {
        let size_x = Vector::from(((size.0 - inner_size.0) * 0.5, size.1));
        let size_y = Vector::from((size.0, (size.1 - inner_size.1) * 0.5));
        CameraCollisionRects {
            top:    CollisionRect::new(
                id,
                (pos.0, pos.1 + size.1 * 0.5 - size_y.1 * 0.5).into(),
                Some(size_y),
            ),
            bottom: CollisionRect::new(
                id,
                (pos.0, pos.1 - size.1 * 0.5 + size_y.1 * 0.5).into(),
                Some(size_y),
            ),
            left:   CollisionRect::new(
                id,
                (pos.0 - size.0 * 0.5 + size_x.0 * 0.5, pos.1).into(),
                Some(size_x),
            ),
            right:  CollisionRect::new(
                id,
                (pos.0 + size.0 * 0.5 - size_x.0 * 0.5, pos.1).into(),
                Some(size_x),
            ),
        }
    }
}
