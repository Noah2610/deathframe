use std::collections::HashMap;
use std::marker::PhantomData;

use amethyst::core::deferred_dispatcher_operation::{
    AddBundle,
    AddSystemDesc,
    DispatcherOperation,
};
use amethyst::core::{ArcThreadPool, SystemBundle, SystemDesc};
use amethyst::ecs::{DispatcherBuilder, System, World, WorldExt};
use amethyst::DataInit;

use super::internal_helpers::*;
use super::CustomGameData;

/// Builder struct for `CustomGameData`.
pub struct CustomGameDataBuilder<'a, 'b, C = ()> {
    core_dispatcher:            DispatcherBuilder<'a, 'b>,
    dispatchers:                HashMap<String, DispatcherBuilder<'a, 'b>>,
    core_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    dispatcher_operations:
        HashMap<String, Vec<Box<dyn DispatcherOperation<'a, 'b>>>>,
    custom:                     Option<C>,
}

impl<'a, 'b, C> CustomGameDataBuilder<'a, 'b, C> {
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
    pub fn custom(mut self, custom: C) -> Self {
        self.custom = Some(custom);
        self
    }

    /// Register a bundle for the _core_ dispatcher.
    pub fn with_core_bundle<B>(mut self, bundle: B) -> amethyst::Result<Self>
    where
        B: SystemBundle<'a, 'b> + 'static,
    {
        self.core_dispatcher_operations
            .push(Box::new(AddBundle { bundle }));
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
        B: SystemBundle<'a, 'b> + 'static,
    {
        let dispatcher_name = dispatcher_name.to_string();
        if self.dispatchers.contains_key(&dispatcher_name) {
            self.dispatcher_operations
                .entry(dispatcher_name)
                .or_insert_with(Vec::new)
                .push(Box::new(AddBundle { bundle }));
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

    /// Register a system description for the _core_ dispatcher.
    pub fn with_core_desc<SD, S>(
        mut self,
        system_desc: SD,
        name: &str,
        dependencies: &[&str],
    ) -> amethyst::Result<Self>
    where
        SD: SystemDesc<'a, 'b, S> + 'static,
        S: for<'c> System<'c> + Send + 'static,
    {
        let name = name.to_string();
        let dependencies =
            dependencies.into_iter().map(ToString::to_string).collect();

        let dispatcher_operation = Box::new(AddSystemDesc {
            system_desc,
            name,
            dependencies,
            marker: PhantomData::<S>,
        })
            as Box<dyn DispatcherOperation<'a, 'b> + 'static>;
        self.core_dispatcher_operations.push(dispatcher_operation);
        Ok(self)
    }

    /// Register a system description for the given dispatcher.
    pub fn with_desc<U, SD, S>(
        mut self,
        dispatcher_name: U,
        system_desc: SD,
        name: &str,
        dependencies: &[&str],
    ) -> amethyst::Result<Self>
    where
        U: ToString,
        SD: SystemDesc<'a, 'b, S> + 'static,
        S: for<'c> System<'c> + Send + 'static,
    {
        let dispatcher_name = dispatcher_name.to_string();
        if self.dispatchers.contains_key(&dispatcher_name) {
            let name = name.to_string();
            let dependencies =
                dependencies.into_iter().map(ToString::to_string).collect();
            let dispatcher_operation = Box::new(AddSystemDesc {
                system_desc,
                name,
                dependencies,
                marker: PhantomData::<S>,
            })
                as Box<dyn DispatcherOperation<'a, 'b> + 'static>;
            self.dispatcher_operations
                .entry(dispatcher_name)
                .or_insert_with(Vec::new)
                .push(dispatcher_operation);
            Ok(self)
        } else {
            Err(dispatcher_not_found(dispatcher_name))
        }
    }
}

impl<'a, 'b, C> DataInit<CustomGameData<'a, 'b, C>>
    for CustomGameDataBuilder<'a, 'b, C>
{
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b, C> {
        // Get handle to the `ThreadPool`
        let pool = (*world.read_resource::<ArcThreadPool>()).clone();

        let mut core_dispatcher_builder = self.core_dispatcher;

        // Build core bundles
        self.core_dispatcher_operations
            .into_iter()
            .try_for_each(|operation| {
                operation.exec(world, &mut core_dispatcher_builder)
            })
            .expect("Couldn't build core bundle");

        // Create core dispatcher
        let mut core_dispatcher =
            core_dispatcher_builder.with_pool(pool.clone()).build();
        core_dispatcher.setup(world);

        let mut dispatcher_operations = self.dispatcher_operations;

        // Create other dispatchers
        let dispatchers = self
            .dispatchers
            .into_iter()
            .map(|(name, mut dispatcher_builder)| {
                // Build bundles
                if let Some(operations) = dispatcher_operations.remove(&name) {
                    operations
                        .into_iter()
                        .try_for_each(|operation| {
                            operation.exec(world, &mut dispatcher_builder)
                        })
                        .expect("Couldn't build bundle");
                }

                // Build dispatcher
                let mut dispatcher =
                    dispatcher_builder.with_pool(pool.clone()).build();
                dispatcher.setup(world);
                (name, dispatcher)
            })
            .collect();

        // Create the `CustomGameData`
        CustomGameData {
            core_dispatcher: Some(core_dispatcher),
            dispatchers,
            custom: self.custom,
        }
    }
}

impl<'a, 'b, C> Default for CustomGameDataBuilder<'a, 'b, C> {
    /// Creates a new builder for `CustomGameData`
    fn default() -> Self {
        Self {
            core_dispatcher:            DispatcherBuilder::new(),
            dispatchers:                HashMap::new(),
            core_dispatcher_operations: Vec::new(),
            dispatcher_operations:      HashMap::new(),
            custom:                     None,
        }
    }
}
