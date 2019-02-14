

use super::point::Point;

pub fn calc_convex_hull(x: Vec<Point>) {
    println!("{:?}", x);
    let mut y = x.to_vec();
    y.sort_by(
        |a, b| { 
            if a.x != b.x {
                return a.x.cmp(&b.x);
            }
            return a.y.cmp(&b.y);
        });
    println!("{:?}", y);

}

