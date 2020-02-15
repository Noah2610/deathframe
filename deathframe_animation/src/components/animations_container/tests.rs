//! Test `AnimationsContainer` component

use crate::components::prelude::*;

#[test]
fn can_build_empty_animations_container() {
    let _ = AnimationsContainer::<()>::builder().build().unwrap();
}

#[test]
fn can_build_animations_container() {
    let _ = AnimationsContainer::<String>::builder()
        .with("FIRST_ANIM".into(), || {
            Box::new(
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
                .into_iter(),
            )
        })
        .with("SECOND_ANIM".into(), || {
            Box::new(
                vec![
                    (1_usize, 500_u64).into(),
                    (1_usize, 500_u64).into(),
                    (1_usize, 500_u64).into(),
                ]
                .into_iter()
                .cycle(),
            )
        })
        .build()
        .unwrap();
}
