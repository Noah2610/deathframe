use crate::components::prelude::Loaded;
use amethyst::ecs::error::Error as SpecsError;
use amethyst::ecs::{Entity, WriteStorage};
use std::collections::HashMap;

/// The load action to perform for an entity.
#[derive(PartialEq)]
enum LoadAction {
    Load,
    Unload,
}

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
        self.load_actions.insert(entity, LoadAction::Load);
    }

    /// The given entity should be unloaded, if it was loaded.
    pub fn unload(&mut self, entity: Entity) {
        // Only unload if it isn't already staged for loading.
        let is_staged_for_loading = self
            .load_actions
            .get(&entity)
            .map(|action| action == &LoadAction::Load)
            .unwrap_or(false);

        if !is_staged_for_loading {
            self.load_actions.insert(entity, LoadAction::Unload);
        }
    }

    /// Run all load actions, with the given `Loaded` storage.
    pub fn run(
        self,
        loadeds: &mut WriteStorage<Loaded>,
    ) -> Result<(), SpecsError> {
        for (entity, load_action) in self.load_actions {
            match load_action {
                LoadAction::Load => {
                    // Load, unless it is already loaded
                    if !loadeds.contains(entity) {
                        loadeds.insert(entity, Loaded)?;
                    }
                }
                LoadAction::Unload => {
                    // Unload, unless it is already unloaded
                    if loadeds.contains(entity) {
                        loadeds.remove(entity);
                    }
                }
            }
        }
        Ok(())
    }
}
