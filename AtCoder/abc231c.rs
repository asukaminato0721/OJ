// 231c.rs
use std::collections::*;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read2() -> (usize, usize) {
    let input = readline();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    (a, b)
}

fn main() {
    let (students_num, querys) = read2();
    let mut heights: Vec<i32> = readline()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    heights.sort_unstable_by(|a, b| b.cmp(a)); // rev
    (0..querys).for_each(|_| {
        let h: i32 = readline().parse().unwrap();
        let mat = heights.binary_search_by(|a| h.cmp(a));
        match mat {
            Ok(i) => println!("{}", i + 1),
            Err(i) => println!("{}", i),
        }
    })
}
