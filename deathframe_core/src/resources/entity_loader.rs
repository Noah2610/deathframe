use crate::components::prelude::Loaded;
use amethyst::ecs::error::Error as SpecsError;
use amethyst::ecs::{Entity, WriteStorage};
use std::collections::HashMap;

/// The load action to perform for an entity.
#[derive(PartialEq)]
enum LoadAction {
    Load,
    Unload,
    Ignore,
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
            .and_modify(|action| {
                if action != &mut LoadAction::Ignore {
                    *action = LoadAction::Load
                }
            })
            .or_insert(LoadAction::Load);
    }

    /// The given entity should be unloaded, if it was loaded.
    pub fn unload(&mut self, entity: Entity) {
        self.load_actions
            .entry(entity)
            .and_modify(|action| {
                if action != &mut LoadAction::Ignore {
                    *action = LoadAction::Unload
                }
            })
            .or_insert(LoadAction::Unload);
    }

    /// The given entity isn't loaded or unloaded.
    pub fn ignore(&mut self, entity: Entity) {
        self.load_actions.insert(entity, LoadAction::Ignore);
    }

    /// Run all load actions, with the given `Loaded` storage.
    pub fn run(
        self,
        loadeds: &mut WriteStorage<Loaded>,
    ) -> Result<(), SpecsError> {
        for (entity, load_action) in self.load_actions {
            match load_action {
                LoadAction::Load => {
                    loadeds.insert(entity, Loaded)?;
                }
                LoadAction::Unload => {
                    loadeds.remove(entity);
                }
                LoadAction::Ignore => (),
            }
        }
        Ok(())
    }
}
