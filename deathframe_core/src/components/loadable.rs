use super::component_prelude::*;

/// Entities which have `Loadable` may be loaded or unloaded
/// (get or remove the `Loaded` component) later on.
#[derive(Default, Component)]
#[storage(VecStorage)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Loadable {
    pub(crate) padding: (Option<f32>, Option<f32>),
}

impl Loadable {
    pub fn with_padding(mut self, padding: (Option<f32>, Option<f32>)) -> Self {
        self.padding = padding;
        self
    }
}
