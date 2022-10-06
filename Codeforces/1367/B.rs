use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().into()
}
fn read_vec() -> Vec<i32> {
    readline().split(' ').map(|x| x.parse().unwrap()).collect()
}

fn main() {
    for _ in 0..readline().parse().unwrap() {
        readline();
        let list = read_vec();
        let even = list
            .iter()
            .enumerate()
            .filter(|(i, j)| i % 2 == 0 && *j % 2 == 1)
            .count();
        let odd = list
            .iter()
            .enumerate()
            .filter(|(i, j)| i % 2 == 1 && *j % 2 == 0)
            .count();
        println!("{}", if even == odd { even as i32 } else { -1 });
    }
}
