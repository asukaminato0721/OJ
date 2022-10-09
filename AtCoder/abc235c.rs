/// abc235_c
use std::collections::HashMap;
use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
fn read_2_number() -> (i32, i32) {
    let binding = readline();
    let mut it = binding.split(' ').into_iter().map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}

fn main() {
    let (N, Q) = read_2_number();
    let mut position: HashMap<i32, Vec<usize>> = HashMap::new();
    readline()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .enumerate()
        .for_each(|(pos, val)| {
            position.entry(val).or_default().push(pos + 1);
        });
    for _ in 0..Q {
        let (xi, ki) = read_2_number();
        if !position.contains_key(&xi) {
            println!("-1");
            continue;
        }
        let pos = position.get(&xi).unwrap();
        if pos.len() < (ki as usize) {
            println!("-1");
            continue;
        }
        println!("{}", pos[ki as usize - 1]);
    }
}
