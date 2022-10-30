use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}

trait My {
    fn abs_diff(&self, other: Self) -> Self;
}
impl My for usize {
    fn abs_diff(&self, b: usize) -> usize {
        if self > &b {
            self - b
        } else {
            b - self
        }
    }
}
fn main() {
    let n_stones = readline().parse::<usize>().unwrap();
    let mut stones = vec![0; n_stones + 10];
    let mut ans = vec![999999999999; n_stones + 10];
    readline()
        .split_ascii_whitespace()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .for_each(|(i, x)| stones[i] = x.parse::<usize>().unwrap());
    ans[1] = 0;
    ans[2] = stones[1].abs_diff(stones[2]);
    // dp[N] is ans
    // dp[i] = min(
    //    dp[i-1]+abs(stone[i] - stone[i-1]),
    //    dp[i-2]+abs(stone[i] - stone[i-2]),
    // )
    for i in 3..=n_stones {
        ans[i] = std::cmp::min(
            ans[i - 1] + stones[i].abs_diff(stones[i - 1]),
            ans[i - 2] + stones[i].abs_diff(stones[i - 2]),
        )
    }
    println!("{}", ans[n_stones]);
}
