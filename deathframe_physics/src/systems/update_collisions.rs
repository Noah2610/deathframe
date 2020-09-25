use super::system_prelude::*;
use std::marker::PhantomData;

const PADDING: (f32, f32) = (2.0, 2.0);

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
pub struct UpdateCollisionsSystem<C>(PhantomData<C>)
where
    C: CollisionTag;

impl<'a, C> System<'a> for UpdateCollisionsSystem<C>
where
    C: 'static + CollisionTag,
{
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Hitbox>,
        WriteStorage<'a, Collider<C>>,
        ReadStorage<'a, Collidable<C>>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            hitboxes,
            mut colliders,
            collidables,
            unloaded_store,
        ): Self::SystemData,
    ) {
        // Generate the collision grid.
        let collision_grid = gen_collision_grid(
            &entities,
            &transforms,
            &hitboxes,
            &collidables,
            &unloaded_store,
            Some(Point::new(PADDING.0, PADDING.1)),
        );

        // Loop through all Colliders, and check for collision in the CollisionGrid.
        for (entity, collider, hitbox, transform, _) in (
            &entities,
            &mut colliders,
            &hitboxes,
            &transforms,
            !&unloaded_store,
        )
            .join()
        {
            let entity_id = entity.id();
            let entity_pos: Point = {
                let trans = transform.translation();
                Point::new(trans.x, trans.y)
            };
            let mut collider_rect = CollisionRect::<C, ()>::builder()
                .id(entity_id)
                .tag(collider.tag.clone())
                .build()
                .unwrap();

            for hitbox_rect in hitbox.rects.iter() {
                let rect = hitbox_rect.clone().with_offset(&entity_pos);
                collider_rect.rects = vec![rect];
                let colliding_rects =
                    collision_grid.colliding_with(&collider_rect);
                if !colliding_rects.is_empty() {
                    let rect_sides = RectSides::new(&collider_rect.rects[0]);
                    for other_rect in colliding_rects {
                        // Check which side is in collision
                        if let Some(side) =
                            other_rect.rects.iter().find_map(|other_rect| {
                                rect_sides.collides_with(other_rect)
                            })
                        {
                            collider.set_collision_with(
                                other_rect.id,
                                side,
                                other_rect.tag.clone(),
                            );
                        }
                    }
                }
            }

            collider.update();
        }
    }
}

struct RectSides {
    outer:  Rect,
    inner:  Rect,
    top:    Rect,
    bottom: Rect,
    left:   Rect,
    right:  Rect,
}

impl RectSides {
    pub fn new(rect: &Rect) -> Self {
        let rect_center = rect.center();
        let base_rect = Rect::builder()
            .top(rect.top - PADDING.1)
            .bottom(rect.bottom + PADDING.1)
            .left(rect.left + PADDING.0)
            .right(rect.right - PADDING.0);

        let inner = base_rect.clone().build().unwrap();
        let top = base_rect
            .clone()
            .top(rect.top)
            .bottom(rect_center.y)
            .build()
            .unwrap();
        let bottom = base_rect
            .clone()
            .bottom(rect.bottom)
            .top(rect_center.y)
            .build()
            .unwrap();
        let left = base_rect
            .clone()
            .left(rect.left)
            .right(rect_center.x)
            .build()
            .unwrap();
        let right = base_rect
            .right(rect.right)
            .left(rect_center.x)
            .build()
            .unwrap();

        Self {
            outer: rect.clone(),
            inner,
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn collides_with(&self, rect: &Rect) -> Option<CollisionSide> {
        use std::convert::TryFrom;

        if !collision_check::do_rects_intersect(&self.outer, rect) {
            return None;
        }

        let colliding_sides = (
            if collision_check::do_rects_intersect(&self.left, rect) {
                Some(CollisionSide::Left)
            } else if collision_check::do_rects_intersect(&self.right, rect) {
                Some(CollisionSide::Right)
            } else {
                None
            },
            if collision_check::do_rects_intersect(&self.top, rect) {
                Some(CollisionSide::Top)
            } else if collision_check::do_rects_intersect(&self.bottom, rect) {
                Some(CollisionSide::Bottom)
            } else {
                None
            },
        );

        if collision_check::do_rects_intersect(&self.inner, rect) {
            Some(CollisionSide::Inner {
                x: colliding_sides
                    .0
                    .and_then(|side| CollisionInnerSideX::try_from(side).ok()),
                y: colliding_sides
                    .1
                    .and_then(|side| CollisionInnerSideY::try_from(side).ok()),
            })
        } else {
            colliding_sides.0.or(colliding_sides.1)
        }
    }
}

impl<C> Default for UpdateCollisionsSystem<C>
where
    C: 'static + CollisionTag,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
