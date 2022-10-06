use std::io;
fn input() -> String {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("error");
    n.trim().into()
}

fn main() {
    for _ in 0..input().parse().unwrap() {
        let k: usize = input().split(' ').nth(1).unwrap().parse().unwrap();
        let mut v: Vec<i64> = input().split(' ').map(|x| x.parse().unwrap()).collect();
        v.sort_unstable();
        println!("{}", v[(v.len() - k - 1)..].into_iter().sum::<i64>());
    }
}
