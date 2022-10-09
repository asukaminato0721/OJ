/// abc224_c
use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
fn read_2_number() -> (i64, i64) {
    let binding = readline();
    let mut it = binding.split(' ').into_iter().map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}
#[derive(Clone, Copy)]
struct P {
    x: i64,
    y: i64,
}
impl P {
    pub fn new(x: i64, y: i64) -> P {
        P { x, y }
    }
}
impl From<(i64, i64)> for P {
    fn from((x, y): (i64, i64)) -> P {
        P::new(x, y)
    }
}
impl Default for P {
    fn default() -> P {
        P::new(0, 0)
    }
}
fn coline(p1: P, p2: P, p3: P) -> bool {
    (p2.y - p1.y) * (p3.x - p2.x) == (p2.x - p1.x) * (p3.y - p2.y)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn line() {
        assert!(coline(P::new(0, 0), P::new(1, 1), P::new(2, 2)));
        assert!(!coline(P::new(0, 0), P::new(1, 1), P::new(2, 3)));
    }
}
/// find the number of triangle in plane
fn main() {
    let n = readline().parse().unwrap();
    let mut ps = vec![P::default(); n as usize];
    for i in 0..n {
        let (x, y) = read_2_number();
        ps[i as usize] = P::new(x, y);
    }
    let mut ans = 0;
    for i in 0..(n-2) {
        for j in (i + 1)..(n-1) {
            for k in (j + 1)..n {
                if !coline(ps[i as usize], ps[j as usize], ps[k as usize]) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
