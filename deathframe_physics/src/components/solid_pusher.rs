use super::component_prelude::*;

#[derive(Component, Default, Deserialize, Clone)]
#[storage(NullStorage)]
#[serde(deny_unknown_fields)]
pub struct SolidPusher;
