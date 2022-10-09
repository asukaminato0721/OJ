/// abc204_c
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

/// build a graph

fn dfs(origin: i32, g: &HashMap<i32, Vec<i32>>, v: i32, visited: &mut Vec<bool>, count: &mut i32) {
    visited[v as usize] = true;
    if origin != v {
        *count += 1;
    }
    if !g.contains_key(&v) {
        return;
    }
    for &next in g.get(&v).unwrap().iter() {
        if visited[next as usize] {
            continue;
        }
        dfs(origin, g, next, visited, count);
    }
}

fn main() {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut ans = 0;
    let (N, M) = read_2_number();
    for _ in 0..M {
        let (a, b) = read_2_number();
        g.entry(a).or_insert(vec![]).push(b);
    }
    for v in g.keys() {
        let mut visited = vec![false; M.max(N) as usize + 10];
        let mut count = 0;
        dfs(*v, &g, *v, &mut visited, &mut count);
        ans += count;
    }
    println!("{}", ans + N);
}
