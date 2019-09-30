use std::collections::HashMap;

use amethyst::core::{ArcThreadPool, SystemBundle};
use amethyst::ecs::{DispatcherBuilder, System};
use amethyst::prelude::*;
use amethyst::DataInit;

use super::internal_helpers::*;
use super::CustomGameData;

struct BundleWrapper<'a, 'b>(pub Box<dyn SystemBundle<'a, 'b>>);
impl<'a, 'b> BundleWrapper<'a, 'b> {
    pub fn new<B>(bundle: B) -> Self
    where
        B: 'static + SystemBundle<'a, 'b>,
    {
        Self(Box::new(bundle))
    }

    pub fn build(
        self: Box<Self>,
        world: &mut World,
        dispatcher: &mut DispatcherBuilder<'a, 'b>,
    ) -> amethyst::Result<()> {
        self.0.build(world, dispatcher)
    }
}

/// Builder struct for `CustomGameData`.
pub struct CustomGameDataBuilder<'a, 'b, C = ()> {
    core_dispatcher: DispatcherBuilder<'a, 'b>,
    dispatchers:     HashMap<String, DispatcherBuilder<'a, 'b>>,
    core_bundles:    Vec<BundleWrapper<'a, 'b>>,
    bundles:         HashMap<String, Vec<BundleWrapper<'a, 'b>>>,
    custom:          Option<C>,
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
        B: 'static + SystemBundle<'a, 'b>,
    {
        self.core_bundles.push(BundleWrapper::new(bundle));
        // bundle.build(world, &mut self.core_dispatcher)?;
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
        B: 'static + SystemBundle<'a, 'b>,
    {
        let dispatcher_name = dispatcher_name.to_string();
        // if let Some(dispatcher) =
        //     self.dispatchers.get_mut(&dispatcher_name.to_string())
        // {
        if self.dispatchers.contains_key(&dispatcher_name) {
            self.bundles
                .entry(dispatcher_name)
                .or_insert(Vec::new())
                .push(BundleWrapper::new(bundle));
            // bundle.build(world, dispatcher)?;
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

impl<'a, 'b, C> DataInit<CustomGameData<'a, 'b, C>>
    for CustomGameDataBuilder<'a, 'b, C>
{
    fn build(mut self, world: &mut World) -> CustomGameData<'a, 'b, C> {
        // Build bundles

        // Get handle to the `ThreadPool`
        let pool = (&*world.read_resource::<ArcThreadPool>().clone()).clone();

        // Build core bundles
        self.core_bundles
            .into_iter()
            .try_for_each(|bundle| {
                BundleWrapper::build(
                    Box::new(bundle),
                    world,
                    &mut self.core_dispatcher,
                )
            })
            .expect("Couldn't build core bundle");

        // Create core dispatcher
        let mut core_dispatcher =
            self.core_dispatcher.with_pool(pool.clone()).build();

        // Create other dispatchers
        core_dispatcher.setup(world);
        let dispatchers = self
            .dispatchers
            .into_iter()
            .map(|(name, dispatcher_builder)| {
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
            core_dispatcher: DispatcherBuilder::new(),
            dispatchers:     HashMap::new(),
            core_bundles:    Vec::new(),
            bundles:         HashMap::new(),
            custom:          None,
        }
    }
}
