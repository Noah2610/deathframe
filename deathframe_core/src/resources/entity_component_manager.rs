use amethyst::ecs::error::Error as SpecsError;
use amethyst::ecs::{Component, Entity, WriteStorage};
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum InsertionAction {
    Insert,
    Remove,
}

impl Default for InsertionAction {
    fn default() -> Self {
        InsertionAction::Insert
    }
}

#[derive(Default)]
pub struct EntityComponentManager {
    prioritize_action: InsertionAction,
    actions:           HashMap<Entity, InsertionAction>,
}

impl EntityComponentManager {
    pub fn with_priority(mut self, prioritize_action: InsertionAction) -> Self {
        self.prioritize_action = prioritize_action;
        self
    }

    pub fn insert(&mut self, entity: Entity) {
        match &self.prioritize_action {
            InsertionAction::Insert => {
                self.actions.insert(entity, InsertionAction::Insert);
            }
            InsertionAction::Remove => {
                self.actions
                    .entry(entity)
                    .or_insert(InsertionAction::Insert);
            }
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        match &self.prioritize_action {
            InsertionAction::Insert => {
                self.actions
                    .entry(entity)
                    .or_insert(InsertionAction::Remove);
            }
            InsertionAction::Remove => {
                self.actions.insert(entity, InsertionAction::Remove);
            }
        }
    }

    pub fn run<C>(self, storage: &mut WriteStorage<C>) -> Result<(), SpecsError>
    where
        C: Component + Default,
    {
        for (entity, action) in self.actions {
            match action {
                InsertionAction::Insert => {
                    storage.insert(entity, C::default())?;
                }
                InsertionAction::Remove => {
                    storage.remove(entity);
                }
            }
        }
        Ok(())
    }
}
