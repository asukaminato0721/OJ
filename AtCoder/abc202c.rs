// 202c.rs
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
    let length: i64 = readline().parse().unwrap();
    let an = read_vec();
    let bn = read_vec();
    let cn = read_vec();
    let mut cnt = vec![0i64; length as usize + 10];
    for &c in cn.iter().skip(1) {
        cnt[(bn[c as usize]) as usize] += 1;
    }
    let mut ans = 0;
    for &a in an.iter().skip(1) {
        ans += cnt[(a) as usize];
    }
    println!("{}", ans);
}
