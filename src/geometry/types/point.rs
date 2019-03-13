#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Point <T> {
    pub x: T,
    pub y: T,
}

//TODO: is this useful? I don't think so? maybe?
impl Point<f64> {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
