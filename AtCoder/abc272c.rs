// 272c.rs
use std::collections::*;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read2() -> (String, u64) {
    let input = readline();
    let mut iter = input.split_whitespace();
    (
        iter.next().unwrap().into(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn main() {
    let N = readline();
    let pn: Vec<i64> = readline()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut odds: Vec<i64> = pn.iter().filter(|&&x| x % 2 == 1).cloned().collect();
    odds.sort_unstable_by(|a, b| b.cmp(a));
    let mut evens: Vec<i64> = pn.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    evens.sort_unstable_by(|a, b| b.cmp(a));
    let oddans = if odds.len() >= 2 {
        odds[0] + odds[1]
    } else {
        -1
    };
    let evenans = if evens.len() >= 2 {
        evens[0] + evens[1]
    } else {
        -1
    };
    match (oddans, evenans) {
        (-1, -1) => println!("-1"),
        (-1, evenans) => println!("{}", evenans),
        (x, -1) => println!("{}", x),
        (x, y) => println!("{}", x.max(y)),
    }
}
