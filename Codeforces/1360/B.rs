use std::io;
fn input() -> String {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("error");
    n.trim().into()
}

fn main() {
    for _ in 0..(input().parse().unwrap()) {
        input();
        let mut list: Vec<i32> = input().split(' ').map(|x| x.parse().unwrap()).collect();
        list.sort_unstable();
        println!("{}", list.windows(2).map(|p| (p[1] - p[0])).min().unwrap());
    }
}
