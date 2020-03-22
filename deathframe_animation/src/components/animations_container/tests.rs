//! Test `AnimationsContainer` component

use crate::components::prelude::*;
use crate::data::prelude::*;

// #[test]
// fn animations_container_from() {
//     let _ = Animations
// }

#[test]
fn can_build_empty_animations_container() {
    let _ = AnimationsContainer::<()>::builder().build().unwrap();
}

#[test]
fn can_build_animations_container() {
    let _ = AnimationsContainer::<String>::builder()
        .with(
            "FIRST_ANIM".into(),
            vec![
                AnimationFrame::builder()
                    .sprite_id(0)
                    .duration_ms(1000)
                    .build()
                    .unwrap(),
                (1_u8, 100_u8).into(),
                (2_usize, 1000_u64).into(),
                (3_usize, 9999999_u64).into(),
            ]
            .into(),
        )
        .with(
            "SECOND_ANIM".into(),
            vec![
                AnimationFrame::from((1_usize, 500_u64)),
                AnimationFrame::from((1_usize, 500_u64)),
                AnimationFrame::from((1_usize, 500_u64)),
            ]
            .into(),
        )
        .with(
            "THIRD_ANIM".into(),
            vec![
                AnimationFrame::from((1_usize, 500_u64)),
                (1_usize, 500_u64).into(),
                (1_usize, 500_u64).into(),
            ]
            .into(),
        )
        .build()
        .unwrap();
}
