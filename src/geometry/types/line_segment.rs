use super::{Point};
use std::cmp;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct LineSegment<T> {
    pub a : Point<T>,
    pub b : Point<T>,
}

fn is_on(a: &Point<i32>, b: &Point<i32>, c: &Point<i32>) -> bool {
   if c.x <= cmp::max(a.x, b.x) && c.x >= cmp::min(a.x, b.x)
      && c.y <= cmp::max(a.y, b.y) && c.y >= cmp::max(a.y, b.y){
       return true;
   }
   return false;
}

impl LineSegment<i32> {

    pub fn intersect(&self, other: &LineSegment<i32>) -> bool {
        let s_irht_a = super::point::is_right_hand_turn(&self.a, &self.b, &other.a);
        let s_irht_b = super::point::is_right_hand_turn(&self.a, &self.b, &other.b);
        
        let o_irht_a = super::point::is_right_hand_turn(&other.a, &other.b, &self.a);
        let o_irht_b = super::point::is_right_hand_turn(&other.a, &other.b, &self.b);

        if s_irht_a != s_irht_b && o_irht_a != o_irht_b {
            return true;
        }

        // Special Cases 
        // p1, q1 and p2 are colinear and p2 lies on segment p1q1 
        if s_irht_a == 0 && is_on(&self.a, &other.a, &self.b) { return true; } 
      
        // p1, q1 and q2 are colinear and q2 lies on segment p1q1 
        if s_irht_b == 0 && is_on(&self.a, &other.b, &self.b) { return true; }
      
        // p2, q2 and p1 are colinear and p1 lies on segment p2q2 
        if o_irht_a == 0 && is_on(&other.a, &self.a, &other.b) { return true; }
      
         // p2, q2 and q1 are colinear and q1 lies on segment p2q2 
        if o_irht_b == 0 && is_on(&other.a, &self.b, &other.b) { return true; }  
            return false;
        }
}
