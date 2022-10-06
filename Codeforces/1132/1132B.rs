use std::io;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("error");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error");
    let mut list: Vec<i64> = line.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    list.sort_unstable();
    let _sum: i64 = list.iter().sum();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error");
    for i in line.trim().split(' ').map(|x| x.parse::<usize>().unwrap()) {
        println!("{}", _sum - list[list.len() - i] as i64);
    }
}
