use super::Point;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct LineSegment<T> {
    pub a : Point<T>,
    pub b : Point<T>,
}