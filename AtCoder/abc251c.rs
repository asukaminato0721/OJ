// 251c.rs
use std::collections::*;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read2() -> (String, u64) {
    let input = readline();
    let mut iter = input.split_whitespace();
    (
        iter.next().unwrap().into(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn main() {
    let Nsubmissions: i32 = readline().parse().unwrap();
    let mut record = HashSet::new();
    let mut max_records = 0;
    let mut ans = 0;
    (1..=Nsubmissions).for_each(|i| {
        let (submit, tscore) = read2();
        if record.insert(submit) && max_records < tscore {
            max_records = tscore;
            ans = i;
        }
    });
    println!("{}", ans);
}
