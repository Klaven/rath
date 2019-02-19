extern crate nalgebra as na;

use super::point::Point;
use na::{Matrix3, LU};

pub fn calc_convex_hull(x: Vec<Point>) {
    println!("{:?}", x);
    let mut y = x.to_vec();

    //Step 1) sort list
    y.sort_by(
        |a, b| { 
            if a.x != b.x {
                return a.x.cmp(&b.x);
            }
            return a.y.cmp(&b.y);
        });
    println!("{:?}", y);


}

fn is_right_hand_turn(p1: Point, p2: Point, p3: Point) -> bool {

    let m = Matrix3::<f64>::new(
        1.0, p1.x as f64, p1.y as f64,
        1.0, p2.x as f64, p2.y as f64,
        1.0, p3.x as f64, p3.y as f64);

    let lu = LU::new(m.clone());

    return lu.determinant() > 0.0;

}

