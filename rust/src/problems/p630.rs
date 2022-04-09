use std::ops::{Add, Sub, Neg};
use std::collections::{HashSet, HashMap};
use crate::utils::euclid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vector2d {
    x: i64,
    y: i64,
}

impl Vector2d {
    fn scalar_mul(self, scalar: i64) -> Vector2d {
        Vector2d { x: scalar * self.x, y: scalar * self.y }
    }

    fn normalize(self) -> Vector2d {
        let (d, _, _) = euclid(self.x, self.y);
        let result = Vector2d { x: self.x / d, y: self.y / d };
        if result.x >= 0 {
            result
        } else {
            -result
        }
    }
}

macro_rules! bin_op {
    ($trait:ident, $op:ident) => {
        impl $trait for Vector2d {
            type Output = Vector2d;

            fn $op(self, rhs: Vector2d) -> Vector2d {
                Vector2d {
                    x: self.x.$op(rhs.x),
                    y: self.y.$op(rhs.y),
                }
            }
        }
    };
}

bin_op!{Add, add}
bin_op!{Sub, sub}

impl Neg for Vector2d {
    type Output = Vector2d;

    fn neg(self) -> Vector2d {
        Vector2d { x: -self.x, y: -self.y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Line {
    start: Vector2d,
    dir: Vector2d,
}

impl Line {
    fn new(start: Vector2d, end: Vector2d) -> Line {
        let dir = (end - start).normalize();
        
        // get normalized start
        let norm_start = if dir.y == 0 {
            // if the line is horizontal, return intersection with the Y axis
            Vector2d { x: 0, y: start.y }
        } else {
            // return the point on the line with the smallest, non-negative y
            let target_y = start.y.rem_euclid(dir.y);
            let t = (target_y - start.y) / dir.y;
            start + dir.scalar_mul(t)
        };

        Line { start: norm_start, dir }
    }
}

fn st(s: i64) -> (i64, i64) {
    (s.pow(2) % 50515093, s % 2000 - 1000)
}

pub fn solve() {
    const N: i32 = 2500;

    let mut points = Vec::new();
    let mut lines = HashSet::new();
    let mut dir_counts = HashMap::new();
    let mut intersections = 0;
    let mut s = st(290797).0;
    for _i in 0..N {
        let (s1, x) = st(s);
        let (s2, y) = st(s1);
        s = s2;
        let p = Vector2d { x, y };
        for &p1 in &points {
            let line = Line::new(p, p1);
            if lines.contains(&line) {
                continue
            }
            let dir_c = dir_counts.entry(line.dir).or_insert(0);
            intersections += 2 * (lines.len() - *dir_c);
            lines.insert(line);
            *dir_c += 1;
        }
        points.push(p);
    }
    println!("M({}) = {}", N, lines.len());
    println!("S({}) = {}", N, intersections);
}
