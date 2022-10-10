// 255c.rs
use std::collections::*;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read4() -> (i64, i64, i64, i64) {
    let input = readline();
    let mut iter = input.split_whitespace();
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn main() {
    let (x, mut initial, mut common_diff, NTerms) = read4();
    let mut fin = initial + common_diff * (NTerms - 1);
    if common_diff < 0 {
        common_diff = -common_diff;
        std::mem::swap(&mut initial, &mut fin);
    }
    if x <= initial {
        let left = (x - initial).abs();
        println!("{}", left);
        return;
    }
    if x >= fin {
        let right = (x - fin).abs();
        println!("{}", right);
        return;
    }
    // 4 7 8
    if common_diff == 0 {
        println!("{}", (x - initial).abs());
        return;
    }
    println!(
        "{}",
        std::cmp::min(
            (x - initial) % common_diff,
            common_diff - (x - initial) % common_diff
        )
    );
}
