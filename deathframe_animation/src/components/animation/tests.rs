//! Test component `Animation`

use super::*;

#[test]
fn animation_from() {
    let _ = Animation::from(vec![
        (0_usize, 100_u64),
        (1_usize, 200_u64),
        (2_usize, 300_u64),
        (3_usize, 400_u64),
        (4_usize, 500_u64),
    ]);
}
