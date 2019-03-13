pub mod convex_hull;
pub mod types;

use self::types::Point as OtherPoint;
use self::types::LineSegment as OtherLineSegment;

// Should I do this for all types to remove the ::types:: requirement?
pub type Point<T> = OtherPoint<T>;
pub type LineSegment<T> = OtherLineSegment<T>;

// TODO: Take vector of points
pub fn convex_hull() {
    println!("calculating the convex hull");

    let x = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 2.0, y: 2.0 },
        Point { x: 4.0, y: 10.0 },
        Point { x: 3.0, y: 2.0 },
        Point { x: 9.0, y: 30.0 },
        Point { x: 21.0, y: 10.0 },
        Point { x: 33.0, y: 21.0 },
        Point { x: 32.0, y: 14.0 },
        Point { x: 10.0, y: 3.0 },
    ];

    //Should return a list of point instead of printing thme out
    convex_hull::calc_convex_hull(x);
}

// TODO: make func that takes file.
