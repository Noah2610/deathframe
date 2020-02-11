//! Test component `Veclocity`

use super::*;

#[test]
fn created_velocity_has_expected_values() {
    let (x, y) = (10.0, 20.0);
    let velocity = Velocity::new(x, y);

    assert_eq!(
        velocity.get(&Axis::X),
        x,
        "Velocity should have expected x value"
    );
    assert_eq!(
        velocity.get(&Axis::Y),
        y,
        "Velocity should have expected y value"
    );
}

#[test]
fn increase_velocity() {
    let (x, y) = (0.0, 0.0);
    let incr = (20.0, 30.0);
    let mut velocity = Velocity::new(x, y);
    velocity.increase(&Axis::X, incr.0);
    velocity.increase(&Axis::Y, incr.1);

    assert_eq!(
        velocity.get(&Axis::X),
        x + incr.0,
        "Velocity should have expected, increased x value"
    );
    assert_eq!(
        velocity.get(&Axis::Y),
        y + incr.1,
        "Velocity should have expected, increased y value"
    );
}

#[test]
fn increase_velocity_with_max() {
    let (x, y) = (0.0, 0.0);
    let incr = (15.0, 15.0);
    let max = (20.0, 20.0);
    let mut velocity = Velocity::new(x, y);
    let increase = |velocity: &mut Velocity| {
        velocity.increase_with_max(&Axis::X, incr.0, max.0);
        velocity.increase_with_max(&Axis::Y, incr.1, max.1);
    };

    increase(&mut velocity);

    assert_eq!(
        velocity.get(&Axis::X),
        x + incr.0,
        "Velocity should have expected, increased x value, below max"
    );
    assert_eq!(
        velocity.get(&Axis::Y),
        y + incr.1,
        "Velocity should have expected, increased y value, below max"
    );

    increase(&mut velocity);

    assert_eq!(
        velocity.get(&Axis::X),
        max.0,
        "Velocity should have expected, increased x value, at max"
    );
    assert_eq!(
        velocity.get(&Axis::Y),
        max.1,
        "Velocity should have expected, increased y value, at max"
    );
}

#[test]
fn clear_velocity() {
    let (x, y) = (10.0, 10.0);
    let mut velocity = Velocity::new(x, y);

    assert_eq!(velocity.get(&Axis::X), x);
    assert_eq!(velocity.get(&Axis::Y), y);

    Axis::for_each(|axis| {
        velocity.clear(&axis);
        assert_eq!(
            velocity.get(&axis),
            0.0,
            "{} velocity should be cleared",
            &axis
        );
    });
}

#[test]
fn clear_all_velocities() {
    let (x, y) = (10.0, 10.0);
    let mut velocity = Velocity::new(x, y);

    assert_eq!(velocity.get(&Axis::X), x);
    assert_eq!(velocity.get(&Axis::Y), y);

    velocity.clear_all();

    Axis::for_each(|axis| {
        assert_eq!(
            velocity.get(&axis),
            0.0,
            "{} velocity should be cleared (clear_all)",
            &axis
        );
    });
}
