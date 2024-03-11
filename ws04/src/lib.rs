#[warn(missing_docs)]
mod coordinate;
mod direction;

#[cfg(test)]
mod tests {
    use crate::coordinate::Coordinate;
    use crate::direction::Direction;

    #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
    // #[test2]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
    fn addition_works_with_2_coordinates() {
        let coord1 = Coordinate::new(1, 0);
        let coord2 = Coordinate::new(0, 1);
        let coord3 = coord1 + coord2;
        assert_eq!(coord3, Coordinate::new(1, 1));
    }

    fn addition_works_with_2_directions() {
        let direct1 = Direction::new(1, 0);
        let direct2 = Direction {x: 0, y: 1}
        let direct3 = direct1 + direct3;
        assert_eq!(direct1 + direct2, Direction::new(1, 1));
    }

    fn addition_works_with_coordinates_and_directions() {
        let coord = Coordinate::new(1, 0);
        let direction = Direction {x: 0, y: 1}
        let coord3 = coord1 + coord2;
        assert_eq!(coord + direction, Coordinate::new(1, 1));
    }

    fn is_within_rectangle() {
        let top_left = Coordinate::new(-3, 3);
        let bottom_right = Coordinate::new(3, -3);
        let coord = Coordinate::new(0, 0)
        assert!(coord.is_within_rectangle(top_left, bottom_right), true);
    }
}

// lib.rs tend to be used as a library, so no main function
// tells compiler if not testing, lib.rs will not be run
