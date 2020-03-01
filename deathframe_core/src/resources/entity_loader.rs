use crate::components::prelude::Loaded;
use amethyst::core::Hidden;
use amethyst::ecs::error::Error as SpecsError;
use amethyst::ecs::{Entity, WriteStorage};
use std::collections::HashMap;

/// The load action to perform for an entity.
#[derive(PartialEq)]
enum LoadAction {
    Load,
    Unload,
    KeepLoaded,
}

// TODO: This isn't really a resource. It's not inserted into the world.
//       It's just used in `EntityLoaderSystem`.

/// The `EntityLoader` is filled with entities that need to be
/// loaded, unloaded, and executes all loading
/// actions at once with the `run` method.
/// _Loading_ takes precedence over _unloading_, so if an entity
/// is staged for loading _and_ unloading, then it will be loaded.
#[derive(Default)]
pub struct EntityLoader {
    load_actions: HashMap<Entity, LoadAction>,
}

impl EntityLoader {
    /// The given entity should be loaded, if it was unloaded.
    pub fn load(&mut self, entity: Entity) {
        self.load_actions
            .entry(entity)
            .and_modify(|action| *action = LoadAction::Load)
            .or_insert(LoadAction::Load);
    }

    /// The given entity should be unloaded, if it was loaded.
    /// Don't unload if it is supposed to be loaded or kept loaded.
    pub fn unload(&mut self, entity: Entity) {
        self.load_actions
            .entry(entity)
            .and_modify(|action| {
                if action != &LoadAction::KeepLoaded
                    && action != &LoadAction::Load
                {
                    *action = LoadAction::Unload
                }
            })
            .or_insert(LoadAction::Unload);
    }

    /// Keep this entity loaded. This doesn't load or unload the entity,
    /// but prevents it from being unloaded.
    /// This is primarily used for performance, so the entity
    /// isn't loaded multiple times, once the EntityLoaderSystem has
    /// already determined, that it is already loaded.
    pub fn keep_loaded(&mut self, entity: Entity) {
        self.load_actions.insert(entity, LoadAction::KeepLoaded);
    }

    /// Run all load actions, with the given `Loaded` storage,
    /// and the optionally given `Hidden` storage.
    pub fn run(
        self,
        loadeds: &mut WriteStorage<Loaded>,
        mut hiddens_opt: Option<&mut WriteStorage<Hidden>>,
    ) -> Result<(), SpecsError> {
        for (entity, load_action) in self.load_actions {
            match load_action {
                LoadAction::Load => {
                    loadeds.insert(entity, Loaded)?;
                    hiddens_opt.as_mut().map(|hiddens| hiddens.remove(entity));
                }
                LoadAction::Unload => {
                    loadeds.remove(entity);
                    hiddens_opt
                        .as_mut()
                        .map(|hiddens| {
                            hiddens.insert(entity, Hidden).map(|_| ())
                        })
                        .unwrap_or(Ok(()))?;
                }
                LoadAction::KeepLoaded => (),
            }
        }
        Ok(())
    }
}
