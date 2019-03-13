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
