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
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Loader>,
        ReadStorage<'a, Loadable>,
        WriteStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            loaders,
            loadables,
            mut loadeds,
        ): Self::SystemData,
    ) {
        let mut entity_loader = EntityLoader::default();

        for (loader, loader_transform) in (&loaders, &transforms).join() {
            let loader_pos = {
                let trans = loader_transform.translation();
                (trans.x, trans.y)
            };
            let in_loading_distance = |target_pos: (f32, f32)| -> bool {
                let dist = (
                    (loader_pos.0 - target_pos.0).abs(),
                    (loader_pos.1 - target_pos.1).abs(),
                );
                dist.0 <= loader.loading_distance.0
                    && dist.1 <= loader.loading_distance.1
            };

            // Unload entities
            for (target_entity, target_transform, _, _) in
                (&entities, &transforms, &loadables, &loadeds).join()
            {
                let target_pos = {
                    let trans = target_transform.translation();
                    (trans.x, trans.y)
                };
                if !in_loading_distance(target_pos) {
                    entity_loader.unload(target_entity)
                }
            }

            // Load entities
            for (target_entity, target_transform, _, _) in
                (&entities, &transforms, &loadables, !&loadeds).join()
            {
                let target_pos = {
                    let trans = target_transform.translation();
                    (trans.x, trans.y)
                };
                if in_loading_distance(target_pos) {
                    entity_loader.load(target_entity)
                }
            }
        }

        entity_loader
            .run(&mut loadeds)
            .expect("EntityLoader didn't run successfully");
    }
}
