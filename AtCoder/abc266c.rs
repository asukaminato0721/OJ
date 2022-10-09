// https://inamori.hateblo.jp/entry/2022/09/23/184653
// abc266c
#![allow(non_snake_case)]

use std::ops::Sub;

fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

//////////////////// Point ////////////////////

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn read() -> Point {
        let v = read_vec();
        Point { x: v[0], y: v[1] }
    }
}

//////////////////// process ////////////////////

fn is_convex(pt1: &Point, pt2: &Point, pt3: &Point) -> bool {
    let v = *pt2 - *pt1;
    let w = *pt3 - *pt2;
    v.x * w.y - v.y * w.x > 0
}

fn is_convex_quad(quad: &Vec<Point>) -> bool {
    (0..4).all(|i| is_convex(&quad[i], &quad[(i + 1) & 3], &quad[(i + 2) & 3]))
}

fn main() {
    let quad = (0..4).map(|_| Point::read()).collect();
    let b = is_convex_quad(&quad);
    println!("{}", if b { "Yes" } else { "No" })
}
