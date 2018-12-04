// A struct with two fields
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn calc_convex_hull(x: Vec<Point>) {
    println!("{:?}", x);
    let y = sort_points(x);
    println!("{:?}", y);
}

pub fn sort_points(x: Vec<Point>) -> Vec<Point> {
    return vec![x[1], x[2]];
}
