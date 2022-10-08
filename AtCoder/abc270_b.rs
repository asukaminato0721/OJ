use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().into()
}
#[allow(dead_code)]
fn read_3_number() -> (i32, i32, i32) {
    let binding = readline();
    let mut it = binding.split(' ').into_iter().map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}
#[allow(dead_code)]
fn read_q() -> Vec<i32> {
    readline()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
#[allow(non_snake_case)]
fn main() {
    let (x, y, z) = read_3_number();
    // 0   y  x
    // x  y  0
    // y 0 x
    if x.abs() < y.abs() || x.signum() * y.signum() < 0 {
        println!("{}", x.abs())
    } else if z.abs() < y.abs() || z.signum() * y.signum() < 0 {
        println!("{}", z.abs() + (x - z).abs())
    } else {
        println!("-1")
    }
}
