use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
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
pub struct CustomGameDataBuilder<'a, 'b, D, C = ()>
where
    D: Hash + Eq + Debug,
{
    core_dispatcher:            DispatcherBuilder<'a, 'b>,
    dispatchers:                HashMap<D, DispatcherBuilder<'a, 'b>>,
    core_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    dispatcher_operations:
        HashMap<D, Vec<Box<dyn DispatcherOperation<'a, 'b>>>>,
    custom:                     Option<C>,
}

impl<'a, 'b, D, C> CustomGameDataBuilder<'a, 'b, D, C>
where
    D: Hash + Eq + Debug,
{
    /// Initialize a new dispatcher with the given name.
    pub fn dispatcher(mut self, name: D) -> amethyst::Result<Self> {
        let name_s = format!("{:?}", &name);

        if self
            .dispatchers
            .insert(name, DispatcherBuilder::new())
            .is_none()
        {
            Ok(self)
        } else {
            Err(amethyst::Error::from_string(format!(
                "A dispatcher with the given name has already been \
                 initialized: {}",
                name_s
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
    pub fn with_bundle<B>(
        mut self,
        dispatcher_name: D,
        bundle: B,
    ) -> amethyst::Result<Self>
    where
        B: SystemBundle<'a, 'b> + 'static,
    {
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
    pub fn with<S>(
        mut self,
        dispatcher_name: D,
        system: S,
        name: &str,
        dependencies: &[&str],
    ) -> amethyst::Result<Self>
    where
        for<'c> S: System<'c> + Send + 'a,
    {
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
    pub fn with_desc<SD, S>(
        mut self,
        dispatcher_name: D,
        system_desc: SD,
        name: &str,
        dependencies: &[&str],
    ) -> amethyst::Result<Self>
    where
        SD: SystemDesc<'a, 'b, S> + 'static,
        S: for<'c> System<'c> + Send + 'static,
    {
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

impl<'a, 'b, D, C> DataInit<CustomGameData<'a, 'b, D, C>>
    for CustomGameDataBuilder<'a, 'b, D, C>
where
    D: Hash + Eq + Debug,
{
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b, D, C> {
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

impl<'a, 'b, D, C> Default for CustomGameDataBuilder<'a, 'b, D, C>
where
    D: Hash + Eq + Debug,
{
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
