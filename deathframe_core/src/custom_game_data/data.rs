use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use amethyst::core::SystemBundle;
use amethyst::ecs::Dispatcher;
use amethyst::prelude::*;
use amethyst::DataDispose;

use super::internal_helpers::*;
use super::CustomGameDataBuilder;

pub struct CustomGameData<'a, 'b, D, C = ()>
where
    D: Hash + Eq + Debug,
{
    pub(crate) core_dispatcher: Option<Dispatcher<'a, 'b>>,
    pub(crate) dispatchers:     HashMap<D, Dispatcher<'a, 'b>>,
    pub custom:                 Option<C>,
}

impl<'a, 'b, D, C> CustomGameData<'a, 'b, D, C>
where
    D: Hash + Eq + Debug,
{
    /// Returns a new `CustomGameDataBuilder` instance.
    pub fn builder<B>() -> CustomGameDataBuilder<'a, 'b, D, C>
    where
        B: SystemBundle<'a, 'b>,
    {
        CustomGameDataBuilder::default()
    }

    // Call this from the active state with the state's (dispatcher's) name every frame.
    pub fn update(
        &mut self,
        world: &World,
        dispatcher_name: D,
    ) -> amethyst::Result<()> {
        if let Some(dispatcher) = self.dispatchers.get_mut(&dispatcher_name) {
            dispatcher.dispatch(&world);
        } else {
            return Err(dispatcher_not_found(dispatcher_name));
        }

        self.update_core(world);

        Ok(())
    }

    pub fn update_core(&mut self, world: &World) {
        self.core_dispatcher
            .as_mut()
            .expect("Core Dispatcher needs to exist when calling update")
            .dispatch(&world);
    }
}

impl<'a, 'b, D, C> DataDispose for CustomGameData<'a, 'b, D, C>
where
    D: Hash + Eq + Debug,
{
    fn dispose(&mut self, world: &mut World) {
        if let Some(dispatcher) = self.core_dispatcher.take() {
            dispatcher.dispose(world);
        }
        self.dispatchers.drain().for_each(|(_name, dispatcher)| {
            dispatcher.dispose(world);
        })
    }
}
