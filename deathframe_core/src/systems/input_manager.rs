use std::hash::Hash;
use std::marker::PhantomData;

use super::system_prelude::*;
use amethyst::input::BindingTypes;

/// Handles all the logic for `InputManager`.
#[derive(Default)]
pub struct InputManagerSystem<B>(PhantomData<B>)
where
    B: BindingTypes + Eq + Default + Hash;

impl<'a, B> System<'a> for InputManagerSystem<B>
where
    B: BindingTypes + Eq + Default + Hash,
{
    type SystemData = (Read<'a, InputHandler<B>>, Write<'a, InputManager<B>>);

    fn run(&mut self, (input_handler, mut input_manager): Self::SystemData) {
        input_manager.update(&input_handler);
    }
}
