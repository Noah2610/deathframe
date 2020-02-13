//! Test component `Animation`

use super::*;

#[test]
fn can_build_animation() {
    let _ = Animation::builder().build().unwrap();
    let _ = Animation::builder()
        .frame(
            AnimationFrame::builder()
                .sprite_id(0)
                .duration_ms(1000)
                .build()
                .unwrap(),
        )
        .frame((1_u8, 100_u8))
        .frame((2_usize, 1000_u64))
        .frame((3_usize, 9999999_u64))
        .build()
        .unwrap();
}
