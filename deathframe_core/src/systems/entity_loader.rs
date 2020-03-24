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
        ReadStorage<'a, Size>,
        ReadStorage<'a, Loader>,
        ReadStorage<'a, Loadable>,
        WriteStorage<'a, Loaded>,
        WriteStorage<'a, Hidden>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            sizes,
            loaders,
            loadables,
            mut loadeds,
            mut hiddens,
        ): Self::SystemData,
    ) {
        let mut entity_loader = EntityLoader::default();

        for (loader, loader_transform) in (&loaders, &transforms).join() {
            let loader_pos = {
                let trans = loader_transform.translation();
                (trans.x, trans.y)
            };
            let in_loading_distance = |target_pos: (f32, f32),
                                       target_loadable: &Loadable,
                                       target_size_opt: Option<&Size>|
             -> bool {
                let dist = {
                    let size = target_size_opt
                        .map(|s| (s.w, s.h))
                        .unwrap_or((0.0, 0.0));
                    let padding = (
                        target_loadable.padding.0.unwrap_or(0.0),
                        target_loadable.padding.1.unwrap_or(0.0),
                    );
                    (
                        ((loader_pos.0 - target_pos.0).abs()
                            - size.0 * 0.5
                            - padding.0),
                        ((loader_pos.1 - target_pos.1).abs()
                            - size.1 * 0.5
                            - padding.1),
                    )
                };

                dist.0 <= loader.loading_distance.0
                    && dist.1 <= loader.loading_distance.1
            };

            for (
                target_entity,
                target_transform,
                target_size_maybe,
                target_loadable,
                target_loaded_maybe,
            ) in (
                &entities,
                &transforms,
                sizes.maybe(),
                &loadables,
                loadeds.maybe(),
            )
                .join()
            {
                let target_pos = {
                    let trans = target_transform.translation();
                    (trans.x, trans.y)
                };

                let is_loaded = target_loaded_maybe.is_some();
                let is_in_loading_distance = in_loading_distance(
                    target_pos,
                    target_loadable,
                    target_size_maybe,
                );

                if is_loaded {
                    if is_in_loading_distance {
                        entity_loader.keep_loaded(target_entity);
                    } else {
                        entity_loader.unload(target_entity);
                    }
                } else {
                    if is_in_loading_distance {
                        entity_loader.load(target_entity);
                    }
                }
            }
        }

        entity_loader
            .run(&mut loadeds, Some(&mut hiddens))
            .expect("EntityLoader didn't run successfully");
    }
}
