use std::default::Default;
use std::ops::Add;
use std::convert::From;
use crate::direction::Direction;
use std::ops::Mul;

#[derive[Debug, PartialEq]]
/// Represent a 2D coordinate.
pub struct Coordinate {
    pub x: i32,
    pub y: i32
}

impl Coordinate {
    /// Create a new coordinate.
    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate {x, y}
    }

    //check that self is within the bounds of the top left corner and bottom
    //right corner of a given rectangle 
    // ```
    // documentation rust
    // ```
    pub fn is_within_rectangle(self, top_left: Coordinate, bottom_right: Coordinate) -> bool {
        // need 4 if statements 
        if self.y <= top_left.y 
            && self.x >= top_left.x 
            && self.y >= bottom_right.y 
            && self.x <= bottom_right.x
    }


    // find the distance to another point
    // Use pythgorean theorem to find the distance between two points
    /// ```
    /// let p1 = Coordinate::new(0, 1);
    /// let p2 = Coordinate::new(-3, -3);
    /// assert_eq!(p1.distance(p2), 5)
    /// ```
    fn distance_from(&self, other: &Self) -> f64 {

        // 
        let y_difference: i32 = self.y - other.y;
        let x_difference: i32 = self.x - other.x;

        let magnitude: f64 = (y_difference) * (y_difference) + (x_difference) * (x_difference) as f64;

        magnitude.sqrt()
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate { x: 0, y: 0}
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    pub fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        Coordinate { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl From<Direction> for Coordinate {
    fn from(dir: Direction) -> Coordinate {
        Coordinate { x: dir.x, y: dir.y }
    }
}


```

impl From<CardinalDirection> for Direction {
    fn from(dir: CardinalDirection) -> Direction {
        match dir {
            CardinalDirection::North => Direction::new(0, 1),
            CardinalDirection::East => Direction::new(1, 0),
            CardinalDirection::South => Direction::new(0, -1),
            CardinalDirection::West => Direction::new(-1, 0),
        }
    }
}

```impl Add<Coordinate> for Direction {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        Direction {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<T> for Direction 
where 
    T: Into<i32>
    T:
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: Direction = rhs.into();
        Direction::new(
            x: self.x * rhs,
            y: self.y * rhs,
        )
        self.x 
    }
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Direction {
        Direction { x, y }
    }
}



// angle definition is different from the math definition 
/// Tells you where a line will end, given a starting point, direction, and length.
/// This is used by `draw_simple_line` to get the end point of a line.
pub fn get_end_coordinates(x: f32, y: f32, direction: i32, length: f32) -> (f32, f32) {
    let x = quantize(x);
    let y = quantize(y);

    // directions start at 0 degrees being straight up, and go clockwise
    // we need to add 90 degrees to make 0 degrees straight right.
    let direction_rad = ((direction as f32) - 90.0).to_radians();

    let end_x = quantize(x + (direction_rad.cos() * length as f32));
    let end_y = quantize(y + (direction_rad.sin() * length as f32));

    (end_x, end_y)
}

fn quantize(x: f32) -> f32 {
    (x * 256.0).round() / 256.0
}