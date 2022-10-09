// abc268_c 
use std::io;
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
fn main() {
    let N: i32 = readline().parse().unwrap();
    let mut P: Vec<i32> = vec![0; N as usize + 10];
    readline()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .for_each(|(i, x)| {
            P[m(x - i as i32, N)] += 1;
            P[m(x - 1 - i as i32, N)] += 1;
            P[m(x + 1 - i as i32, N)] += 1;
        });
    println!("{}", P.iter().max().unwrap());
}
