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
        ReadStorage<'a, CollidableCustomData>,
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
            collidable_custom_data_store,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        // Generate the collision grid.
        let collision_grid = gen_collision_grid(GenCollisionGridData {
            entities:                 &entities,
            transforms:               &transforms,
            hitboxes:                 &hitboxes,
            with_collision_tag_comps: &collidables,
            loadables:                &loadables,
            loadeds:                  &loadeds,
            padding:                  Some(Point::new(PADDING.0, PADDING.1)),
            _c:                       Default::default(),
        });

        // Loop through all Colliders, and check for collision in the CollisionGrid.
        for (entity, collider, hitbox, transform) in
            (&entities, &mut colliders, &hitboxes, &transforms).join()
        {
            if is_entity_loaded(entity, &loadables, &loadeds) {
                let entity_id = entity.id();
                let entity_pos: Point = {
                    let trans = transform.translation();
                    Point::new(trans.x, trans.y)
                };
                let collider_base_rect = CollisionRect::<C, ()>::builder()
                    .id(entity_id)
                    .tag(collider.tag.clone());

                for hitbox_rect in hitbox.rects.iter() {
                    let rect = hitbox_rect.clone().with_offset(&entity_pos);
                    let collider_rect =
                        collider_base_rect.clone().rect(rect).build().unwrap();
                    let colliding_rects =
                        collision_grid.colliding_with(&collider_rect);
                    if !colliding_rects.is_empty() {
                        let rect_sides = RectSides::new(&collider_rect.rect);
                        for other_rect in colliding_rects {
                            // Check which side is in collision
                            if let Some(side) =
                                rect_sides.collides_with(&other_rect.rect)
                            {
                                collider.set_collision_with(
                                    other_rect.id.expect(
                                        "`CollisionRect` should have an `id` \
                                         here",
                                    ),
                                    side,
                                    other_rect.tag.clone().expect(
                                        "`CollisionRect` should have a \
                                         `CollisionTag` here",
                                    ),
                                );
                            }
                        }
                    }
                }

                collider.update();
            }
        }
    }
}

struct RectSides {
    pub inner:  Rect,
    pub top:    Rect,
    pub bottom: Rect,
    pub left:   Rect,
    pub right:  Rect,
}

impl RectSides {
    pub fn new(rect: &Rect) -> Self {
        let base_rect = Rect::builder()
            .top(rect.top - PADDING.1)
            .bottom(rect.bottom + PADDING.1)
            .left(rect.left + PADDING.0)
            .right(rect.right - PADDING.0);

        let inner = base_rect.clone().build().unwrap();
        let top = base_rect.clone().top(rect.top).build().unwrap();
        let bottom = base_rect.clone().bottom(rect.bottom).build().unwrap();
        let left = base_rect.clone().left(rect.left).build().unwrap();
        let right = base_rect.clone().right(rect.right).build().unwrap();

        Self {
            inner,
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn collides_with(&self, rect: &Rect) -> Option<CollisionSide> {
        if CollisionGrid::<(), ()>::do_rects_intersect(rect, &self.inner) {
            Some(CollisionSide::Inner)
        } else if CollisionGrid::<(), ()>::do_rects_intersect(rect, &self.top) {
            Some(CollisionSide::Top)
        } else if CollisionGrid::<(), ()>::do_rects_intersect(
            rect,
            &self.bottom,
        ) {
            Some(CollisionSide::Bottom)
        } else if CollisionGrid::<(), ()>::do_rects_intersect(rect, &self.left)
        {
            Some(CollisionSide::Left)
        } else if CollisionGrid::<(), ()>::do_rects_intersect(rect, &self.right)
        {
            Some(CollisionSide::Right)
        } else {
            None
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
