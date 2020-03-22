//! Test component `Animation`

use super::*;

#[test]
fn animation_from() {
    let _ = Animation::from(vec![
        (0, 1000),
        (1, 1000),
        (2, 500),
        (3, 500),
        (4, 500),
    ]);
}

#[test]
fn can_build_animation_with_frames() {
    let _ = Animation::builder()
        .frames(Box::new(
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
            .into_iter()
            .cycle(),
        ))
        .build()
        .unwrap();
}

#[test]
#[should_panic]
fn can_not_build_animation_without_frames() {
    let _ = Animation::builder().build().unwrap();
}
