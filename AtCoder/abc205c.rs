// 205c.rs
use std::collections::*;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read3() -> (i32, i32, i32) {
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
    let (mut base1, mut base2, pow) = read3();
    if pow % 2 == 0 {
        base1 = base1.abs();
        base2 = base2.abs();
    }
    match base1.cmp(&base2) {
        std::cmp::Ordering::Less => println!("<"),
        std::cmp::Ordering::Equal => println!("="),
        std::cmp::Ordering::Greater => println!(">"),
    }
}
