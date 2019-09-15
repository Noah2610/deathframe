use std::collections::HashMap;

use amethyst::core::{ArcThreadPool, SystemBundle};
use amethyst::ecs::{DispatcherBuilder, System};
use amethyst::prelude::*;
use amethyst::DataInit;

use super::internal_helpers::*;
use super::CustomGameData;

/// Builder struct for `CustomGameData`.
pub struct CustomGameDataBuilder<'a, 'b, T = ()> {
    core_dispatcher: DispatcherBuilder<'a, 'b>,
    dispatchers:     HashMap<String, DispatcherBuilder<'a, 'b>>,
    custom:          Option<T>,
}

impl<'a, 'b, T> CustomGameDataBuilder<'a, 'b, T> {
    /// Creates a new builder for `CustomGameData`
    pub fn new() -> Self {
        Self {
            core_dispatcher: DispatcherBuilder::new(),
            dispatchers:     HashMap::new(),
            custom:          None,
        }
    }

    /// Initialize a new dispatcher with the given name.
    pub fn dispatcher<U>(mut self, name: U) -> amethyst::Result<Self>
    where
        U: ToString,
    {
        if self
            .dispatchers
            .insert(name.to_string(), DispatcherBuilder::new())
            .is_none()
        {
            Ok(self)
        } else {
            Err(amethyst::Error::from_string(format!(
                "A dispatcher with the given name has already been \
                 initialized: {}",
                name.to_string()
            )))
        }
    }

    /// Store some optional custom data.
    pub fn custom(mut self, custom: T) -> Self {
        self.custom = Some(custom);
        self
    }

    /// Register a bundle for the _core_ dispatcher.
    pub fn with_core_bundle<B>(mut self, bundle: B) -> amethyst::Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle.build(&mut self.core_dispatcher)?;
        Ok(self)
    }

    /// Register a bundle for the given dispatcher.
    pub fn with_bundle<U, B>(
        mut self,
        dispatcher_name: U,
        bundle: B,
    ) -> amethyst::Result<Self>
    where
        U: ToString,
        B: SystemBundle<'a, 'b>,
    {
        let dispatcher_name = dispatcher_name.to_string();
        if let Some(dispatcher) =
            self.dispatchers.get_mut(&dispatcher_name.to_string())
        {
            bundle.build(dispatcher)?;
            Ok(self)
        } else {
            Err(dispatcher_not_found(dispatcher_name))
        }
    }

    /// Register a system for the _core_ dispatcher.
    /// Returns `Self` wrapped in a `Result`, just for consistency.
    /// This method will _always_ return `Ok`.
    pub fn with_core<S>(
        mut self,
        system: S,
        name: &str,
        dependencies: &[&str],
    ) -> amethyst::Result<Self>
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.core_dispatcher.add(system, name, dependencies);
        Ok(self)
    }

    /// Register a system for the given dispatcher.
    pub fn with<U, S>(
        mut self,
        dispatcher_name: U,
        system: S,
        name: &str,
        dependencies: &[&str],
    ) -> amethyst::Result<Self>
    where
        U: ToString,
        for<'c> S: System<'c> + Send + 'a,
    {
        let dispatcher_name = dispatcher_name.to_string();
        if let Some(dispatcher) = self.dispatchers.get_mut(&dispatcher_name) {
            dispatcher.add(system, name, dependencies);
            Ok(self)
        } else {
            Err(dispatcher_not_found(dispatcher_name))
        }
    }
}

impl<'a, 'b, T> DataInit<CustomGameData<'a, 'b, T>>
    for CustomGameDataBuilder<'a, 'b, T>
{
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b, T> {
        // Get handle to the `ThreadPool`
        let pool = world.read_resource::<ArcThreadPool>().clone();

        // Create core dispatcher
        let mut core_dispatcher =
            self.core_dispatcher.with_pool(pool.clone()).build();

        // Create other dispatchers
        core_dispatcher.setup(&mut world.res);
        let dispatchers = self
            .dispatchers
            .into_iter()
            .map(|(name, dispatcher_builder)| {
                let mut dispatcher =
                    dispatcher_builder.with_pool(pool.clone()).build();
                dispatcher.setup(&mut world.res);
                (name, dispatcher)
            })
            .collect();

        // Create the `CustomGameData`
        CustomGameData {
            core_dispatcher,
            dispatchers,
            custom: self.custom,
        }
    }
}

impl<'a, 'b, T> Default for CustomGameDataBuilder<'a, 'b, T> {
    fn default() -> Self {
        Self::new()
    }
}
