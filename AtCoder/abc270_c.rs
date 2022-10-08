use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().into()
}
#[allow(dead_code)]
fn read_3_number() -> (i32, i32, i32) {
    let binding = readline();
    let mut it = binding.split(' ').map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}
fn read_2_number() -> (usize, usize) {
    let binding = readline();
    let mut it = binding.split(' ').map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}
#[allow(dead_code)]
fn read_q() -> Vec<i32> {
    readline()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
#[allow(non_snake_case)]
fn main() {
    let (n, x, y) = read_3_number();
    let mut to: Vec<Vec<usize>> = vec![vec![]; n as usize + 1];
    for _ in 0..(n-1) {
        let (p, q) = read_2_number();
        to[p as usize].push(q);
        to[q as usize].push(p);
    }
    let mut ans: Vec<usize> = vec![];
    fn dfs(v: usize, p: usize, x: i32, ans: &mut Vec<usize>, to: &Vec<Vec<usize>>) -> bool {
        if v == x as usize {
            ans.push(v);
            return true;
        }
        for u in to[v].iter() {
            if *u == p {
                continue;
            };
            if dfs(*u, v, x, ans, to) {
                ans.push(v);
                return true;
            }
        }
        false
    }
    dfs(y as usize, 0, x, &mut ans, &to);
    ans.iter().for_each(|x| print!("{} ", x));
}
