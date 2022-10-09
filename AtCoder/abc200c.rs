// 200c
use std::{collections::*, vec};
fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}
fn readline() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.into()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
/// only same last two && all odd or all even pairs could count.
#[derive(Clone, Copy, Debug)]
struct T {
    odd: u64,
    even: u64,
}
fn main() {
    readline();
    let mut same_last_two = vec![T { odd: 0, even: 0 }; 101];
    readline()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .for_each(|x| {
            let hundred_digits = (x / 100) % 10;
            if hundred_digits % 2 == 0 {
                same_last_two[(x % 100) as usize].even += 1;
            } else {
                same_last_two[(x % 100) as usize].odd += 1;
            }
        });
    println!(
        "{}",
        same_last_two
            .iter()
            .map(|&T { even, odd }| {
                let e = if even >= 2 { even * (even - 1) / 2 } else { 0 };
                let o = if odd >= 2 { odd * (odd - 1) / 2 } else { 0 };
                e + o
            })
            .sum::<u64>()
    );
}
