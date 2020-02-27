use super::system_prelude::*;

/// The `EntityLoaderSystem` handles the loading and unloading
/// of entities. Entities with the `Loader` component load
/// entities when they are in range with `Loadable` entities,
/// and `Loadable` entities are unloaded when _no_ `Loader` entities
/// are in range.
#[derive(Default)]
pub struct EntityLoaderSystem;

impl<'a> System<'a> for EntityLoaderSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, EntityLoader>,
        // ReadStorage<'a, Loader>, // TODO: Loader component
        ReadStorage<'a, Loadable>,
        WriteStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            entity_loader,
            // loaders,
            loadables,
            mut loadeds,
        ): Self::SystemData,
    ) {
    }
}
