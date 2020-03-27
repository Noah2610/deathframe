use super::system_prelude::*;
use crate::resources::entity_component_inserter::InsertionAction;

/// The `EntityLoaderSystem` handles the loading and unloading
/// of entities. Entities with the `Loader` component load
/// entities when they are in range with `Loadable` entities,
/// and `Loadable` entities are unloaded when _no_ `Loader` entities
/// are in range.
pub struct EntityLoaderSystem {
    entity_loader:        EntityComponentInserter,
    entity_loader_hidden: EntityComponentInserter,
}

impl Default for EntityLoaderSystem {
    fn default() -> Self {
        Self {
            entity_loader:        EntityComponentInserter::default()
                .with_priority(InsertionAction::Insert)
                .with_cache(true),
            entity_loader_hidden: EntityComponentInserter::default()
                .with_priority(InsertionAction::Remove)
                .with_cache(true),
        }
    }
}

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
        for (loader, loader_transform) in (&loaders, &transforms).join() {
            let loader_pos = {
                let trans = loader_transform.translation();
                (trans.x, trans.y)
            };
            let is_in_distance = |target_pos: (f32, f32),
                                  target_loadable: &Loadable,
                                  target_size_opt: Option<&Size>|
             -> (bool, bool) {
                let (loading_distance, render_distance) = {
                    let size = target_size_opt
                        .map(|s| (s.w, s.h))
                        .unwrap_or((0.0, 0.0));
                    let padding = (
                        target_loadable.padding.0.unwrap_or(0.0),
                        target_loadable.padding.1.unwrap_or(0.0),
                    );
                    let render_distance = (
                        ((loader_pos.0 - target_pos.0).abs() - size.0 * 0.5),
                        ((loader_pos.1 - target_pos.1).abs() - size.1 * 0.5),
                    );
                    let loading_distance = (
                        render_distance.0 - padding.0,
                        render_distance.1 - padding.1,
                    );
                    (loading_distance, render_distance)
                };

                let in_loading_distance = loading_distance.0
                    <= loader.loading_distance.0
                    && loading_distance.1 <= loader.loading_distance.1;
                let in_render_distance = render_distance.0
                    <= loader.loading_distance.0
                    && render_distance.1 <= loader.loading_distance.1;

                (in_loading_distance, in_render_distance)
            };

            for (
                target_entity,
                target_transform,
                target_size_maybe,
                target_loadable,
            ) in (&entities, &transforms, sizes.maybe(), &loadables).join()
            {
                let target_pos = {
                    let trans = target_transform.translation();
                    (trans.x, trans.y)
                };

                let (in_loading_distance, in_render_distance) = is_in_distance(
                    target_pos,
                    target_loadable,
                    target_size_maybe,
                );

                if in_loading_distance {
                    self.entity_loader.insert(target_entity);
                } else {
                    self.entity_loader.remove(target_entity);
                }

                if in_render_distance {
                    self.entity_loader_hidden.remove(target_entity);
                } else {
                    self.entity_loader_hidden.insert(target_entity);
                }
            }
        }

        self.entity_loader
            .run(&mut loadeds)
            .expect("EntityLoader didn't load entities successfully");
        self.entity_loader_hidden
            .run(&mut hiddens)
            .expect("EntityLoader didn't show/hide entities successfully");
    }
}
