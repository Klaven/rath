extern crate nalgebra as na;

use super::types::Point;
use na::{Matrix3, LU};

pub fn calc_convex_hull(x: Vec<Point<f64>>) {
    println!("{:?}", x);
    let mut y = x.to_vec();

    //Step 1) sort list
    y.sort_by(|a, b| {
        if a.x != b.x {
            return a.x.partial_cmp(&b.x).unwrap();
        }
        return a.y.partial_cmp(&b.y).unwrap();
    });
    println!("{:?}", y);

    let mut upper = Vec::<Point<f64>>::new();
    let mut lower = Vec::<Point<f64>>::new();

    upper.push(y[0].clone());
    upper.push(y[1].clone());

    for i in 2..y.len() {
        if !is_right_hand_turn(&upper[upper.len() - 2], &upper[upper.len() - 1], &y[i]) {
            upper.remove(upper.len() - 1);
            if upper.len() < 2 || i == y.len() - 1 {
                upper.push(y[i].clone());
            }
        } else {
            upper.push(y[i].clone());
        }
    }

    y.reverse();

    lower.push(y[0].clone());
    lower.push(y[1].clone());

    for i in 2..y.len() {
        if !is_right_hand_turn(&lower[lower.len() - 2], &lower[lower.len() - 1], &y[i]) {
            lower.remove(lower.len() - 1);
            if lower.len() < 2 || i == y.len() - 1 {
                lower.push(y[i].clone());
            }
        } else {
            lower.push(y[i].clone());
        }
    }

    lower.remove(lower.len() - 1);
    lower.remove(0);

    upper.append(&mut lower);

    println!("results: {:?}", upper);
}

fn is_right_hand_turn(p1: &Point<f64>, p2: &Point<f64>, p3: &Point<f64>) -> bool {
    let m = Matrix3::<f64>::new(
        1.0,
        p1.x,
        p1.y,
        1.0,
        p2.x as f64,
        p2.y as f64,
        1.0,
        p3.x as f64,
        p3.y as f64,
    );

    let lu = LU::new(m.clone());

    return lu.determinant() > 0.0;
}
