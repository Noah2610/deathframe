use super::system_prelude::*;

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
        {}
    }
}
