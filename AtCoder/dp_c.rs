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
    a: usize,
    b: usize,
    c: usize,
}
impl From<(usize, usize, usize)> for Thing {
    fn from((a, b, c): (usize, usize, usize)) -> Self {
        Self { a, b, c }
    }
}
fn main() {
    let n_days: usize = readline().parse().unwrap();
    let mut things = vec![Thing { a: 0, b: 0, c: 0 }; n_days as usize + 10];
    for i in 1..=n_days {
        things[i] = read3().into();
    }
    // dp[i][n] means ith day and nth activity max happiness
    // dp[i][0] = max(dp[i-1][1], dp[i-1][2]) + things[i][n] where m != n
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n_days as usize + 10];
    dp[1][0] = things[1].a;
    dp[1][1] = things[1].b;
    dp[1][2] = things[1].c;
    for i in 2..=n_days as usize {
        dp[i][0] = max(dp[i - 1][1], dp[i - 1][2]) + things[i].a;
        dp[i][1] = max(dp[i - 1][0], dp[i - 1][2]) + things[i].b;
        dp[i][2] = max(dp[i - 1][0], dp[i - 1][1]) + things[i].c;
    }
    println!("{}", dp[n_days].iter().max().unwrap());
}
