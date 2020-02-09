use super::component_prelude::*;

/// A `Hitbox` has one or more `Rect` rects,
/// which are collision boxes, relative to this entity's `Transform`.
/// So the `Rect` rects assume the entity's position is at `0, 0`.
#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Hitbox {
    pub(crate) rects: Vec<Rect>,
}

impl Hitbox {
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.add_rect(rect);
        self
    }

    pub fn add_rect(&mut self, rect: Rect) {
        self.rects.push(rect);
    }
}
