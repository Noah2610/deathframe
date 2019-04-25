use super::system_prelude::*;

/// Handles all the logic for `InputManager`.
pub struct InputManagerSystem;

impl<'a> System<'a> for InputManagerSystem {
    type SystemData = (
        Read<'a, InputHandler<String, String>>,
        Write<'a, InputManager>,
    );

    fn run(&mut self, (input_handler, mut input_manager): Self::SystemData) {
        input_manager.update(&input_handler);
    }
}
