use std::collections::{HashMap, HashSet};
use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
fn read_2_number() -> (usize, usize) {
    let bind = readline();
    let mut it = bind.split_whitespace().map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}
fn read_num_str() -> (usize, String) {
    let bind = readline();
    let mut it = bind.split_ascii_whitespace();
    (
        it.next().unwrap().parse().unwrap(),
        it.next().unwrap().into(),
    )
}
fn main() {
    let (N, M) = read_2_number();
    let mut ac = vec![false; 100010];
    let mut ac_num = 0;
    let mut wa = vec![0; 100010];
    let mut wa_num = 0;
    for _ in 0..M {
        let (id, res) = read_num_str();
        match (ac[id], res.as_str()) {
            (true, _) => {}
            (false, "AC") => {
                ac[id] = true;
                ac_num += 1;
                wa_num += wa[id];
            }
            (false, "WA") => {
                wa[id] += 1;
            }
            (_, _) => {}
        }
    }
    print!("{} {}", ac_num, wa_num);
}