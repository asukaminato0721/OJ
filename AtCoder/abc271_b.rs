use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().into()
}
fn read_2_number() -> (u32, u32) {
    let binding = readline();
    let mut it = binding.split(' ').into_iter().map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}
#[allow(dead_code)]
fn read_vec() -> Vec<i32> {
    readline().split(' ').map(|x| x.parse().unwrap()).collect()
}
#[allow(non_snake_case)]
fn main() {
    let (N, Q) = read_2_number();
    let mut arr: Vec<Vec<i32>> = vec![vec![]; 200010];
    for i in 0..N {
        arr[i as usize] = readline()
            .split(' ')
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect()
    }
    for _ in 0..Q {
        let (s, t) = read_2_number();
        println!("{}", arr[s as usize - 1][t as usize - 1]);
    }
}
