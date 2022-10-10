// 217c.rs
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
    let pn = readline()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .enumerate()
        .map(|(i, x)| (x, (i + 1) as i32))
        .collect::<HashMap<i32, i32>>();
    for i in 1..=N.parse::<i32>().unwrap() {
        print!("{} ", pn[&i]);
    }
}
