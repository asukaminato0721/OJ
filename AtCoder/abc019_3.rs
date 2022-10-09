use std::collections::HashSet;
use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
fn main() {
    readline();
    let mut record: HashSet<i32> = HashSet::new();
    readline()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .for_each(|x| {
            let mut x = x;
            while x % 2 == 0 {
                x /= 2;
            }
            record.insert(x);
        });
    println!("{}", record.len())
}