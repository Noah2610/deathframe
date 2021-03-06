use super::component_prelude::*;

/// Entities that have `Confined` and at least a `Transform`,
/// are confined to the confined `Rect`'s area, with the `ConfineEntitiesSystem`.
#[derive(Component, Builder, Deserialize)]
#[storage(VecStorage)]
#[builder(pattern = "owned")]
#[serde(deny_unknown_fields)]
pub struct Confined {
    pub(crate) rect: Rect,
}

impl Confined {
    pub fn builder() -> ConfinedBuilder {
        ConfinedBuilder::default()
    }
}

impl<R> From<R> for Confined
where
    R: Into<Rect>,
{
    fn from(rect: R) -> Self {
        Self { rect: rect.into() }
    }
}
