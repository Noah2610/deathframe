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

/// This struct accumulates `InsertionAction`s for entities,
/// and then _inserts_ or _removes_ a certain component from all
/// entities at once, by calling the `run` method.
/// Component to insert/remove has to have `Default` implemented.
/// It can prioritize insertion or removal over the other action.
/// Used with `EntityLoaderSystem`.
#[derive(Default)]
pub struct EntityComponentInserter {
    prioritize_action: InsertionAction,
    actions:           HashMap<Entity, InsertionAction>,
    prev_actions:      Option<HashMap<Entity, InsertionAction>>,
}

impl EntityComponentInserter {
    /// Which `InsertionAction` to prioritize.
    /// So if the `Insert` action is prioritized, then no subsequent
    /// `Remove` action can ever overwrite the `Insert` action. And vice versa.
    /// See `InsertionAction`'s `Default` implementation for the default.
    pub fn with_priority(mut self, prioritize_action: InsertionAction) -> Self {
        self.prioritize_action = prioritize_action;
        self
    }

    /// If caching logic should be used.
    /// The caching is pretty bad, and seems to slow things down,
    /// rather than speed stuff up. Not recommended.
    pub fn with_cache(mut self, use_cache: bool) -> Self {
        if use_cache {
            self.prev_actions = Some(Default::default());
        } else {
            self.prev_actions = None;
        }
        self
    }

    /// Stage the given entity for _insertion_.
    /// So on `run`, the given component will be _added_ to the entity.
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

    /// Stage the given entity for _removal_.
    /// So on `run`, the given component will be _removed_ from the entity.
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

    /// Run all `InsertionAction`s at once.
    /// When inserting, inserts the `Default` component
    /// of the given component storage.
    pub fn run<C>(
        &mut self,
        storage: &mut WriteStorage<C>,
    ) -> Result<(), SpecsError>
    where
        C: Component + Default,
    {
        for (entity, action) in self.actions.drain() {
            let prev_action = self
                .prev_actions
                .as_ref()
                .and_then(|prev| prev.get(&entity));

            match (action, prev_action) {
                // Do nothing, if the new action is the same one as previously.
                (InsertionAction::Insert, Some(InsertionAction::Insert)) => (),
                (InsertionAction::Remove, Some(InsertionAction::Remove)) => (),

                (action, _) => {
                    match &action {
                        InsertionAction::Insert => {
                            storage.insert(entity, C::default())?;
                        }
                        InsertionAction::Remove => {
                            storage.remove(entity);
                        }
                    }

                    if let Some(prev_actions) = self.prev_actions.as_mut() {
                        prev_actions.insert(entity, action);
                    }
                }
            }
        }

        Ok(())
    }
}
