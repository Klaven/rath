pub mod convex_hull;
pub mod point;

use self::point::Point;

pub fn convex_hull() {
    println!("calculating the convex hull");

    //Should read in file or something.
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

    convex_hull::calc_convex_hull(x);
}
