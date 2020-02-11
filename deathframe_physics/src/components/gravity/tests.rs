//! Test component `Gravity`

use super::*;

#[test]
fn can_build_gravity_with_no_values() {
    let _gravity = Gravity::builder().build().unwrap();
}

#[test]
fn built_gravity_has_expected_values() {
    let (x, y) = (10.0, -20.0);
    let gravity = Gravity::builder().x(x).y(y).build().unwrap();

    assert_eq!(
        gravity.get(&Axis::X),
        Some(x),
        "Gravity should have expected x value"
    );
    assert_eq!(
        gravity.get(&Axis::Y),
        Some(y),
        "Gravity should have expected y value"
    );
}

#[test]
fn can_set_gravity_values() {
    let (x, y) = (20.0, -100.0);
    let mut gravity = Gravity::default();

    Axis::for_each(|axis| assert_eq!(gravity.get(&axis), None));

    gravity.set(&Axis::X, x);
    assert_eq!(gravity.get(&Axis::X), Some(x));
    assert_eq!(gravity.get(&Axis::Y), None);
    gravity.set(&Axis::Y, y);
    assert_eq!(gravity.get(&Axis::X), Some(x));
    assert_eq!(gravity.get(&Axis::Y), Some(y));
}
