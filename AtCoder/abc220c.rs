// 220c.rs
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
    let an = read_vec();
    let X: i64 = readline().parse().unwrap();
    let sum_a = an.iter().sum::<i64>();
    let times = X / sum_a;
    let mut ans = times * N;
    let mut rest = X % sum_a;
    for a in an {
        if rest >= a {
            rest -= a;
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
