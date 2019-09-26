use std::collections::HashMap;

use amethyst::core::SystemBundle;
use amethyst::ecs::Dispatcher;
use amethyst::prelude::*;
use amethyst::DataDispose;

use super::internal_helpers::*;
use super::CustomGameDataBuilder;

pub struct CustomGameData<'a, 'b, C = ()> {
    pub(crate) core_dispatcher: Option<Dispatcher<'a, 'b>>,
    pub(crate) dispatchers:     HashMap<String, Dispatcher<'a, 'b>>,
    pub custom:                 Option<C>,
}

impl<'a, 'b, C> CustomGameData<'a, 'b, C> {
    /// Returns a new `CustomGameDataBuilder` instance.
    pub fn builder<B>() -> CustomGameDataBuilder<'a, 'b, C>
    where
        B: SystemBundle<'a, 'b>,
    {
        CustomGameDataBuilder::default()
    }

    // Call this from the active state with the state's (dispatcher's) name every frame.
    pub fn update<U>(
        &mut self,
        world: &World,
        dispatcher_name: U,
    ) -> amethyst::Result<()>
    // TODO: Create proper error enum
    where
        U: ToString,
    {
        let dispatcher_name = dispatcher_name.to_string();

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

impl<'a, 'b, C> DataDispose for CustomGameData<'a, 'b, C> {
    fn dispose(&mut self, world: &mut World) {
        if let Some(dispatcher) = self.core_dispatcher.take() {
            dispatcher.dispose(world);
        }
        self.dispatchers.drain().for_each(|(_name, dispatcher)| {
            dispatcher.dispose(world);
        })
    }
}
