//! Coordinate geometry basics.
//!
//! Orientation and area formulas are the basic predicates behind many geometry algorithms.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::coordinate_geometry_basics::{orientation, Point};
//!
//! let a = Point { x: 0, y: 0 };
//! let b = Point { x: 1, y: 0 };
//! let c = Point { x: 1, y: 1 };
//! assert_eq!(orientation(a, b, c), 1);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub fn orientation(a: Point, b: Point, c: Point) -> i64 {
    let cross = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    cross.signum()
}

#[cfg(test)]
mod tests {
    use super::{orientation, Point};

    #[test]
    fn orientation_detects_turn_direction() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 1, y: 1 };
        assert_eq!(orientation(a, b, c), 1);
    }
}

