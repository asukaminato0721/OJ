use std::collections::HashSet;
use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().into()
}
#[allow(dead_code)]
fn read_2_number() -> (u32, u32) {
    let binding = readline();
    let mut it = binding.split(' ').into_iter().map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}
#[allow(dead_code)]
fn read_q() -> HashSet<i32> {
    readline()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
#[allow(non_snake_case)]
fn main() {
    let n: i32 = readline().parse().unwrap();
    let s = read_q();
    let mut max = 0;
    let mut sum = 0;
    for i in 1.. {
        if s.contains(&i) {
            sum += 1;
        } else {
            sum += 2;
        }
        if sum > n {
            break;
        }
        max = i;
    }
    println!("{}", max);
}
