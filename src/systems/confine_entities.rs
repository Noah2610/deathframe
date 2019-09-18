use super::system_prelude::*;
use crate::geo::{Rect, RectBuilder, Vector};

/// This system confines all entities with `Transform` and `Confined`
/// to the rect defined in `Confined`, taking `Size` into account.
pub struct ConfineEntitiesSystem;

impl<'a> System<'a> for ConfineEntitiesSystem {
    type SystemData = (
        ReadStorage<'a, Confined>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (confineds, sizes, loadables, loadeds, mut transforms): Self::SystemData,
    ) {
        for (confined, size_opt, transform, loadable_opt, loaded_opt) in (
            &confineds,
            sizes.maybe(),
            &mut transforms,
            loadables.maybe(),
            loadeds.maybe(),
        )
            .join()
        {
            if let (None, None) | (Some(_), Some(_)) =
                (loadable_opt, loaded_opt)
            {
                let pos = Vector::from(&*transform);
                if let Some(size) = size_opt {
                    let rect = RectBuilder::default()
                        .with_pos_and_size(pos, size.into())
                        .build();
                    if rect.left < confined.rect.left {
                        transform.set_translation_x(
                            confined.rect.left + size.w * 0.5,
                        );
                    }
                    if rect.right > confined.rect.right {
                        transform.set_translation_x(
                            confined.rect.right - size.w * 0.5,
                        );
                    }
                    if rect.bottom < confined.rect.bottom {
                        transform.set_translation_y(
                            confined.rect.bottom + size.h * 0.5,
                        );
                    }
                    if rect.top > confined.rect.top {
                        transform.set_translation_y(
                            confined.rect.top - size.h * 0.5,
                        );
                    }
                } else {
                    transform.set_translation_x(
                        pos.0.min(confined.rect.right).max(confined.rect.left),
                    );
                    transform.set_translation_y(
                        pos.1.min(confined.rect.top).max(confined.rect.bottom),
                    );
                }
            }
        }
    }
}
