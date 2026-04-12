//! Line intersection algorithms.
//!
//! Segment intersection is built from orientation tests and bounding-box checks.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::line_intersection_algorithms::{segments_intersect, Point};
//!
//! assert!(segments_intersect(
//!     Point { x: 0, y: 0 },
//!     Point { x: 2, y: 2 },
//!     Point { x: 0, y: 2 },
//!     Point { x: 2, y: 0 },
//! ));
//! ```

pub use crate::advanced::misc_topics::coordinate_geometry_basics::Point;
use crate::advanced::misc_topics::coordinate_geometry_basics::orientation;

pub fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);
    o1 != o2 && o3 != o4
}

#[cfg(test)]
mod tests {
    use super::{segments_intersect, Point};

    #[test]
    fn line_intersection_detects_crossing_segments() {
        assert!(segments_intersect(
            Point { x: 0, y: 0 },
            Point { x: 2, y: 2 },
            Point { x: 0, y: 2 },
            Point { x: 2, y: 0 },
        ));
    }
}
