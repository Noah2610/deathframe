use super::component_prelude::*;

/// `Size` is used in multiple places, including collision and scaling.
#[derive(Debug)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}

impl Component for Size {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32)> for Size {
    fn from(data: (f32, f32)) -> Self {
        Self::new(data.0, data.1)
    }
}
