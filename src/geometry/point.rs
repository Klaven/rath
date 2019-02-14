
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd,Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64
}


impl Point {
    pub fn new(x:i64, y: i64) -> Self {
        Point {
            x,
            y
        }
    }
}


