// 207c.rs
use std::collections::*;
use std::f64;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read3() -> (i64, i64, i64) {
    let input = readline();
    let mut iter = input.split_whitespace();
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}
fn read_vec() -> Vec<i32> {
    std::iter::once(0)
        .chain(
            readline()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap()),
        )
        .collect()
}
fn main() {
    let N: i64 = readline().parse().unwrap();
    let mut lr: Vec<(f64, f64)> = (0..N)
        .map(|_| read3())
        .map(|(t, l, r)| match t {
            1 => (l as f64, r as f64),
            2 => (l as f64, r as f64 - 0.5),
            3 => (l as f64 + 0.5, r as f64),
            4 => (l as f64 + 0.5, r as f64 - 0.5),
            _ => unreachable!(),
        })
        .collect();
    lr.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut cnt = 0i64;
    for (i, (l1, r1)) in lr.iter().take(N as usize - 1).enumerate() {
        for (l2, r2) in lr.iter().skip(i + 1) {
            if (l1.max(*l2)) <= r1.min(*r2) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
