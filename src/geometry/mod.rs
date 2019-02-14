pub mod convex_hull;
pub mod point;

use self::point::Point;

pub fn convex_hull() {
    println!("calculating the convex hull");

    //Should read in file or something.
    let x = vec![
        Point { x: 1, y: 1 },
        Point { x: 2, y: 2 },
        Point { x: 4, y: 10 },
        Point { x: 3, y: 2 },
        Point { x: 9, y: 30 },
        Point { x: 21, y: 10 },
        Point { x: 33, y: 21 },
        Point { x: 10, y: 3 },
    ];

    convex_hull::calc_convex_hull(x);
}
