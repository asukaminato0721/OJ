// abc267_c
use std::{io, iter};
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
fn m(x: i32, modd: i32) -> usize {
    let mut x = x;
    while x < 0 {
        x += modd;
    }
    (x % modd) as usize
}
fn read2() -> (i64, i64) {
    let bind = readline();
    let mut it = bind.split_ascii_whitespace().map(|x| x.parse().unwrap());

    (it.next().unwrap(), it.next().unwrap())
}
fn main() {
    let (N, M) = read2();
    let arr: Vec<i64> = readline()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let presum = iter::once(&0)
        .chain(&arr)
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect::<Vec<i64>>();
    let mut cur = arr
        .iter()
        .take(M as usize)
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .map(|(i, x)| i as i64 * x)
        .map(|x|x as i64)
        .sum::<i64>();
    let mut max = cur;
    for i in M..N {
        let loss = presum[(i) as usize] - presum[(i - M) as usize];
        let gain = arr[i as usize] * M;
        
        cur = cur - (loss) + (gain);
        max = max.max(cur);
    }
    println!("{}", max);
}
