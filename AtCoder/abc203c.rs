// 203c.rs
use std::collections::*;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read2() -> (u64, u64) {
    let input = readline();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    (a, b)
}
struct S {
    pos: u64,
    money: u64,
}
fn main() {
    let (friends_num, mut Kyuan) = read2();
    let mut friends_with_money: Vec<S> = (0..friends_num)
        .map(|_| read2())
        .map(|(k, v)| S { pos: k, money: v })
        .collect();
    friends_with_money.sort_unstable_by_key(|s| s.pos);
    let mut cur_pos = 0;
    for S { pos, money } in friends_with_money {
        if Kyuan >= pos - cur_pos {
            Kyuan -= pos - cur_pos;
            Kyuan += money;
            cur_pos = pos;
        } else {
            break;
        }
    }
    println!("{}", Kyuan + cur_pos);
}
