use super::component_prelude::*;
use super::Size;

/// `InnerSize` is a newtype with `Size`.
/// It is only used by `Camera` (for now).
pub struct InnerSize(pub Size);

impl Component for InnerSize {
    type Storage = HashMapStorage<Self>;
}
