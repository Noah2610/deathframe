use super::system_prelude::*;
use crate::geo::prelude::*;

const PADDING: f32 = 1.0;

/// The `CollisionSystem` is in charge of setting collision states for colliding entities.
/// Entities with `CheckCollision` (and with `Collision`) check for collision against
/// other entities with `Collision`.

// NOTE:
// Consider giving `CollisionSystem` a `CollisionGrid` field, which stores the generated
// `CollisionGrid` between frames; then only update `CollisionRect`s within the grid, which do not
// move (which do not have a `Velocity`).
// This might improve performance, as the `CollisionGrid` wouldn't be re-generated every frame.
// It would have to re-generate and remove all `CollisionRect`s with moving entities each frame
// though, so benchmarking would be needed to verify that this would be beneficial.
pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, CheckCollision>,
        WriteStorage<'a, Collision>,
    );

    fn run(
        &mut self,
        (entities, transforms, sizes, check_collisions, mut collisions): Self::SystemData,
    ) {
        let collision_grid = CollisionGrid::new(
            (&entities, &transforms, sizes.maybe(), &mut collisions)
                .join()
                .map(|(entity, transform, size_opt, _)| {
                    let entity_id = entity.id();
                    let pos = transform.translation();
                    // Create a CollisionRect with increased size, for touch collision checking
                    let rect = CollisionRect::with_custom(
                        entity_id,
                        (pos.x - PADDING, pos.y - PADDING).into(),
                        size_opt.map(|size| {
                            (size.w + PADDING, size.h + PADDING).into()
                        }),
                        None,
                    );
                    rect
                })
                .collect::<Vec<CollisionRect<()>>>(),
        );

        for (entity, collision, _) in
            (&entities, &mut collisions, &check_collisions).join()
        {
            if let Some(rect) = collision_grid.rect_by_id(entity.id()) {
                let colliding = collision_grid.colliding_with(rect);
                if !colliding.is_empty() {
                    let rect_side_rects =
                        create_collision_rects_for_sides_from(rect);
                    for other_rect in colliding {
                        // Check which side is in collision
                        if let Some(side) =
                            rect_side_rects.collides_with_side(other_rect)
                        {
                            collision.set_collision_with(other_rect.id, side);
                        }
                    }
                }
            }

            collision.update();
        }
    }
}

struct CollisionRectSides {
    pub inner:  CollisionRect<Side>,
    pub top:    CollisionRect<Side>,
    pub bottom: CollisionRect<Side>,
    pub left:   CollisionRect<Side>,
    pub right:  CollisionRect<Side>,
}

impl CollisionRectSides {
    pub fn collides_with_side<T>(
        &self,
        target_rect: &CollisionRect<T>,
    ) -> Option<Side> {
        let expect_msg =
            "`CollisionRect` (for sides) should have custom data with `Side`";
        if CollisionGrid::<()>::do_rects_collide(target_rect, &self.inner) {
            Some(self.inner.custom.expect(expect_msg))
        } else if CollisionGrid::<()>::do_rects_collide(target_rect, &self.top)
        {
            Some(self.top.custom.expect(expect_msg))
        } else if CollisionGrid::<()>::do_rects_collide(
            target_rect,
            &self.bottom,
        ) {
            Some(self.bottom.custom.expect(expect_msg))
        } else if CollisionGrid::<()>::do_rects_collide(target_rect, &self.left)
        {
            Some(self.left.custom.expect(expect_msg))
        } else if CollisionGrid::<()>::do_rects_collide(
            target_rect,
            &self.right,
        ) {
            Some(self.right.custom.expect(expect_msg))
        } else {
            None
        }
    }
}

fn create_collision_rects_for_sides_from<T>(
    rect: &CollisionRect<T>,
) -> CollisionRectSides {
    CollisionRectSides {
        inner:  CollisionRect {
            id:     rect.id,
            top:    rect.top - PADDING,
            bottom: rect.bottom + PADDING,
            left:   rect.left + PADDING,
            right:  rect.right - PADDING,
            custom: Some(Side::Inner),
        },
        top:    CollisionRect {
            id:     rect.id,
            top:    rect.top,
            bottom: rect.bottom + PADDING,
            left:   rect.left + PADDING,
            right:  rect.right - PADDING,
            custom: Some(Side::Top),
        },
        bottom: CollisionRect {
            id:     rect.id,
            top:    rect.top - PADDING,
            bottom: rect.bottom,
            left:   rect.left + PADDING,
            right:  rect.right - PADDING,
            custom: Some(Side::Bottom),
        },
        left:   CollisionRect {
            id:     rect.id,
            top:    rect.top - PADDING,
            bottom: rect.bottom + PADDING,
            left:   rect.left,
            right:  rect.right - PADDING,
            custom: Some(Side::Left),
        },
        right:  CollisionRect {
            id:     rect.id,
            top:    rect.top - PADDING,
            bottom: rect.bottom + PADDING,
            left:   rect.left + PADDING,
            right:  rect.right,
            custom: Some(Side::Right),
        },
    }
}
