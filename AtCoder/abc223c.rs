// 223c.rs
use std::collections::*;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read2() -> (i64, i64) {
    let input = readline();
    let mut iter = input.split_whitespace();
    (
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
    let N: i32 = readline().parse().unwrap();
    let AB: Vec<(i64, i64)> = (0..N).map(|_| read2()).collect();
    let unify: Vec<f64> = AB.iter().map(|(a, b)| (*a as f64 / *b as f64)).collect();
    let mut half_sum = unify.iter().sum::<f64>() / 2.0;
    let mut nth = 0;
    while half_sum > 0f64 {
        half_sum -= unify[nth as usize];
        nth += 1;
    }
    nth -= 1;
    let a: f64 = AB.iter().take(nth).map(|(aa, bb)| *aa as f64).sum();
    let ans = a as f64 + (half_sum + unify[nth as usize]) * AB[nth as usize].1 as f64;
    println!("{}", ans);
}
