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
fn main() {
    let (n_stones, k) = read2();
    let mut stones = vec![0; n_stones + 10];
    let mut ans = vec![9999999999; n_stones + 10];
    readline()
        .split_ascii_whitespace()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .for_each(|(i, x)| stones[i] = x.parse::<i64>().unwrap());
    ans[1] = 0;
    // dp[N] is ans
    // dp[i] = min(
    //    dp[i-1]+abs(stone[i] - stone[i-1]),
    //    dp[i-2]+abs(stone[i] - stone[i-2]),
    // )
    for i in 2..=n_stones {
        ans[i] = (1..=k)
            .filter(|&j| i >= j)
            .map(|j| ans[i - j] + (stones[i] - stones[i - j]).abs())
            .min() //
            .unwrap();
    }
    println!("{}", ans[n_stones]);
}
