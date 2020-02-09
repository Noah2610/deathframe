//! TODO: Rewrite this...

use super::system_prelude::*;
use std::marker::PhantomData;

/// TODO, don't know what to do with this value.
const PADDING: (f32, f32) = (1.0, 1.0);

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
        let padding = Point::new(PADDING.0, PADDING.1);

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

                grid.append(
                    // Create the CollisionRect(s) for this entity.
                    // Multiple CollisionRects may exist, because an entity
                    // can have multiple Hitboxes (Hitbox parts).
                    hitbox
                        .rects
                        .iter()
                        .map(|hitbox_rect| {
                            let coll_rect = hitbox_rect
                                .clone()
                                .with_offset(&entity_pos)
                                .with_padding(&padding);
                            base_collision_rect.clone().rect(coll_rect).build()
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
        // Generate the collision grid.
        let collision_grid = Self::get_collision_grid(
            &entities,
            &transforms,
            &hitboxes,
            &collidables,
            &loadables,
            &loadeds,
        );

        // Loop through all Colliders, and check for collision in the CollisionGrid.
        for (entity, collider) in (&entities, &mut colliders).join() {
            if is_entity_loaded(entity, &loadables, &loadeds) {
                let entity_id = entity.id();

                if let Some(coll_rect) = collision_grid.rect_by_id(entity_id) {
                    let colliding_rects =
                        collision_grid.colliding_with(coll_rect);
                    if !colliding_rects.is_empty() {
                        let rect_sides = RectSides::new(&coll_rect.rect);
                        for other_rect in colliding_rects {
                            // Check which side is in collision
                            if let Some(side) =
                                rect_sides.collides_with(&other_rect.rect)
                            {
                                unimplemented!()
                                // TODO
                                // collider.set_collision_with(
                                //     other_rect.id.expect(
                                //         "`CollisionRect` should have an `id` \
                                //          here",
                                //     ),
                                //     side,
                                // );
                            }
                        }
                    }
                }

                unimplemented!();
                // TODO
                // collider.update();
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
