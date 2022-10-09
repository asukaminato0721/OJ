// 263c
fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read2() -> (usize, usize) {
    let v = read_vec::<usize>();
    (v[0], v[1])
}

fn dfs(digits: usize, upper: usize, cur: &mut Vec<i32>, pos: usize, start_value: usize) {
    if pos == digits {
        cur.iter().take(digits).for_each(|x| print!("{} ", x));
        println!();
    }
    // will not overflow
    // if some value is bigger than upper, it will be skipped.
    for i in (start_value + 1)..=upper {
        cur[pos] = i as i32;
        dfs(digits, upper, cur, pos + 1, i);
    }
}
fn main() {
    let (digits, upper) = read2();
    let mut v: Vec<i32> = vec![0; digits + 10];
    dfs(digits, upper, &mut v, 0, 0);
}
