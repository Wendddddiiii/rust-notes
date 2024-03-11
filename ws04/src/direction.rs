
use crate::coordinate::Coordinate;
use std::ops::Ads;


pub struct Direction {
    pub x: i32,
    pub y: i32
}


impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

impl From<Direction> for CardinalDirection {
    fn from(dir: CardinalDirection) -> Direction {
        match dir {
            CardinalDirection::North => Direction::new(0, 1),
            CardinalDirection::East => Direction::new(1, 0),
            CardinalDirection::South => Direction::new(0, -1),
            CardinalDirection::West => Direction::new(-1, 0),


        }
    }
}