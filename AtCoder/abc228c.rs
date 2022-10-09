use std::collections::*;
fn readline() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().into()
}
fn read2() -> (usize, usize) {
    let input = readline();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    (a, b)
}
fn read_students_sum_mark() -> i32 {
    let s = readline();
    s.split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .sum()
}
fn main() {
    let (students_num, kdays) = read2();
    let sums = (0..students_num)
        .map(|_| read_students_sum_mark())
        .collect::<Vec<i32>>();
    let mut sum_sorted = sums.clone();
    
   
    sum_sorted.sort_unstable_by(|a,b| b.cmp(a));
    let kth = sum_sorted[kdays-1];
    for i in sums {
        if i + 300 >= kth {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
