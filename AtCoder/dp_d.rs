use std::cmp::max;
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
    let (Nthings, Weight) = read2();
    let things = (0..Nthings)
        .map(|_| read2())
        .map(Thing::from)
        .collect::<Vec<_>>();
    let mut dp = vec![vec![0; Weight + 1]; Nthings + 1];
    for i in 0..Nthings {
        for j in 0..=Weight {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            if j + things[i].w <= Weight {
                dp[i + 1][j + things[i].w] = max(dp[i + 1][j + things[i].w], dp[i][j] + things[i].v);
            }
        }
    }
    println!("{}", dp[Nthings][Weight]);
}
