//! Test `AnimationsContainer` component

use crate::components::prelude::*;

#[test]
fn can_build_animations_container() {
    let _ = AnimationsContainer::<()>::builder().build().unwrap();
}
