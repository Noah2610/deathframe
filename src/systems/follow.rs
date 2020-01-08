use super::system_prelude::*;

#[derive(Default)]
pub struct FollowSystem;

impl<'a> System<'a> for FollowSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
    }
}
