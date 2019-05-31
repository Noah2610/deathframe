use super::component_prelude::*;

/// Entities' `Transform`s are _confined_ to an area with this component.
/// Their `Transform` may never leave this confined area.
pub struct Confined {}
