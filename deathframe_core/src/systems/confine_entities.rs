use super::system_prelude::*;

/// This system confines all entities with `Transform` and `Confined`
/// to the rect defined in `Confined`, taking `Size` into account.
#[derive(Default)]
pub struct ConfineEntitiesSystem;

impl<'a> System<'a> for ConfineEntitiesSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Confined>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            confined_store,
            mut transforms,
            sizes,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        for (_, confined, transform, size_opt) in
            (&entities, &confined_store, &mut transforms, sizes.maybe())
                .join()
                .filter(|(entity, _, _, _)| {
                    is_entity_loaded(*entity, &loadables, &loadeds)
                })
        {
            let pos = {
                let trans = transform.translation();
                Point::new(trans.x, trans.y)
            };
            if let Some(size) = size_opt {
                let rect = Rect::from(size).with_offset(&pos);
                let half_size = size.half();
                if rect.left < confined.rect.left {
                    transform
                        .set_translation_x(confined.rect.left + half_size.w);
                }
                if rect.right > confined.rect.right {
                    transform
                        .set_translation_x(confined.rect.right - half_size.w);
                }
                if rect.bottom < confined.rect.bottom {
                    transform
                        .set_translation_y(confined.rect.bottom + half_size.h);
                }
                if rect.top > confined.rect.top {
                    transform
                        .set_translation_y(confined.rect.top - half_size.h);
                }
            } else {
                transform.set_translation_x(
                    pos.x.min(confined.rect.right).max(confined.rect.left),
                );
                transform.set_translation_y(
                    pos.y.min(confined.rect.top).max(confined.rect.bottom),
                );
            }
        }
    }
}
