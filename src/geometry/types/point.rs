extern crate nalgebra as na;

use na::{Matrix3, LU};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Point <T> {
    pub x: T,
    pub y: T,
}

//TODO: is this useful? I don't think so? maybe?
impl <T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// checks if is a right hand turn
// 0 == colinear
// 1 == right hand
// 2 == Counter Clockwise
pub fn is_right_hand_turn(p1: &Point<i32>, p2: &Point<i32>, p3: &Point<i32>) -> i32 {
    let m = Matrix3::<f32>::new(
        1.0,
        p1.x as f32,
        p1.y as f32,
        1.0,
        p2.x as f32,
        p2.y as f32,
        1.0,
        p3.x as f32,
        p3.y as f32,
    );

    let lu = LU::new(m.clone());
    let res = lu.determinant();

    if res == 0.0
    {
        return 0
    }
        
    return if res == 0.0 {0} else if res > 0.0 {1} else {2}
}
