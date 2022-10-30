// 209c.rs
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
fn read_vec() -> Vec<i64> {
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
    let mut an = read_vec();
    an.sort_unstable();
    let mut ans: i64 = 1;
    let m = 1000000007;
    for (i, a) in an.iter().skip(1).enumerate() {
        ans *= (a - i as i64);
        ans %= m;
    }
    println!("{}", ans);
}
