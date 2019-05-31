use super::system_prelude::*;
use crate::geo::{Rect, RectBuilder, Vector};

/// This system confines all entities with `Transform` and `Confined`
/// to the rect defined in `Confined`, taking `Size` into account.
pub struct ConfineEntitiesSystem;

impl<'a> System<'a> for ConfineEntitiesSystem {
    type SystemData = (
        ReadStorage<'a, Confined>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (confineds, sizes, mut transforms): Self::SystemData) {
        for (confined, size_opt, transform) in
            (&confineds, sizes.maybe(), &mut transforms).join()
        {
            let pos = Vector::from(&*transform);
            if let Some(size) = size_opt {
                let rect = RectBuilder::default()
                    .with_pos_and_size(pos, size.into())
                    .build();
                if rect.left < confined.rect.left {
                    transform.set_x(confined.rect.left + size.w * 0.5);
                }
                if rect.right > confined.rect.right {
                    transform.set_x(confined.rect.right - size.w * 0.5);
                }
                if rect.bottom < confined.rect.bottom {
                    transform.set_y(confined.rect.bottom + size.h * 0.5);
                }
                if rect.top > confined.rect.top {
                    transform.set_y(confined.rect.top - size.h * 0.5);
                }
            } else {
                transform.set_x(
                    pos.0.min(confined.rect.right).max(confined.rect.left),
                );
                transform.set_y(
                    pos.1.min(confined.rect.top).max(confined.rect.bottom),
                );
            }
        }
    }
}
