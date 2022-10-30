use std::cmp::max;
use std::collections::HashMap;
use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
fn read2() -> (usize, usize) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}
fn read3() -> (usize, usize, usize) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Thing {
    w: usize,
    v: usize,
}
impl From<(usize, usize)> for Thing {
    fn from((w, v): (usize, usize)) -> Self {
        Self { w, v }
    }
}
fn main() {
    let (N, M) = read2();
    let mut G: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 1..=M {
        let (a, b) = read2();
        G.entry(a).or_insert_with(Vec::new).push(b);
        G.entry(b).or_insert_with(Vec::new).push(a);
    }
    let cnt = G
        .iter()
        .filter(|(v, edges)| {
            edges
                .iter() //
                .filter(|&e| e < v)
                .count()
                == 1
        })
        .count();
    println!("{}", cnt);
}
