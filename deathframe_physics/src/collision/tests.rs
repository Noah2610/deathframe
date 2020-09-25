//! Test module `deathframe_physics::collision`

/// `CollisionRect` tests
mod collision_rect_tests {
    use crate::collision::prelude::*;
    use core::geo::prelude::*;
    use std::collections::HashMap;

    /// Returns two Rects that intersect each other
    fn get_intersecting_rects() -> (Rect, Rect) {
        //   01234567
        // 4 +----+
        // 3 |  +-+-+
        // 2 |  | | |
        // 1 +--+-+ |
        // 0    +---+
        let one = Rect::builder()
            .top(4.0)
            .bottom(1.0)
            .left(0.0)
            .right(5.0)
            .build()
            .unwrap();
        let two = Rect::builder()
            .top(3.0)
            .bottom(0.0)
            .left(3.0)
            .right(7.0)
            .build()
            .unwrap();
        (one, two)
    }

    fn get_intersecting_collision_rects(
    ) -> (CollisionRect<(), ()>, CollisionRect<(), ()>) {
        let colliding_rects = get_intersecting_rects();
        let one = CollisionRect::builder()
            .rect(colliding_rects.0)
            .id(0)
            .tag(())
            .build()
            .unwrap();
        let two = CollisionRect::builder()
            .rect(colliding_rects.1)
            .id(1)
            .tag(())
            .build()
            .unwrap();
        (one, two)
    }

    #[test]
    #[should_panic]
    fn can_not_build_collision_rect_with_only_rect() {
        let rect = Rect::default();
        let _collision_rect = CollisionRect::<(), ()>::builder()
            .rect(rect)
            .build()
            .expect("Couldn't build CollisionRect with only Rect");
    }

    #[test]
    #[should_panic]
    fn can_not_build_collision_rect_with_nothing() {
        let _collision_rect =
            CollisionRect::<(), ()>::builder().build().unwrap();
    }

    #[test]
    fn rects_do_intersect() {
        let (one, two) = get_intersecting_collision_rects();
        assert!(
            collision_check::do_rects_collide(&one, &two),
            "CollisionRects should intersect"
        );
    }

    #[test]
    fn rects_collide_in_collision_rect() {
        let (one, two) = get_intersecting_collision_rects();
        let mut grid_map = HashMap::new();
        grid_map.insert("One", one.clone());
        grid_map.insert("Two", two.clone());
        let grid = CollisionGrid::new(grid_map);

        assert!(
            grid.collides_any(&one),
            "First CollisionRect should collide with any other"
        );
        assert!(
            grid.collides_any(&two),
            "Second CollisionRect should collide with any other"
        );
        let colliding_with_one = grid.colliding_with(&one);
        assert_eq!(
            colliding_with_one.len(),
            1,
            "First CollisionRect should be colliding with one other \
             CollisionRect in CollisionGrid"
        );
        assert_eq!(
            &colliding_with_one[0].rects[0], &two.rects[0],
            "CollisionRect colliding with first Rect should be the second Rect"
        );
    }
}
