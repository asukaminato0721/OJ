// 206c.rs
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
    let Nelements: i32 = readline().parse().unwrap();
    let an = read_vec();
    let mut cnt: HashMap<i32,i32> = HashMap::new();
    for &i in an.iter().skip(1) {
        *cnt.entry(i).or_insert(0) += 1;
    }
    let mut ans = 0i64;
    for &i in cnt.values() {
        ans += (Nelements - i) as i64 * i as i64;
    }
    println!("{}", ans / 2);
}
