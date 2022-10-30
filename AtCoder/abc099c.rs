use std::{io, iter::successors};
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}

fn division(x: usize, y: usize) -> usize {
    successors(Some(1), |x| Some(x * y))
        .find(|&i| i > x)
        .unwrap()
        / y
}
fn main() {
    let n: usize = readline().parse().unwrap();

    // dp[i] = min(dp[i], dp[i - division(i, 6)] + 1, dp[i - division(i, 9)] + 1)
    let mut dp = vec![1; (n + 1) as usize];
    dp[0] = 0;
    dp[1] = 1;
    for i in 1..=n {
        dp[i as usize] = std::cmp::min(dp[i - division(i, 6)], dp[i - division(i, 9)]) + 1;
    }
    println!("{}", dp[n]);
}
