use super::component_prelude::*;

#[derive(Component, Default, Builder)]
#[storage(VecStorage)]
#[builder(pattern = "owned")]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
