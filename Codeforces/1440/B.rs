use std::{io, iter};
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().into()
}
fn return_n_k() -> (usize, usize) {
    let binding = readline();
    let mut it = binding
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .take(2);

    (it.next().unwrap(), it.next().unwrap())
}

fn main() {
    for _ in 0..readline().parse().unwrap() {
        let (n, k) = return_n_k();
        let binding = readline();
        let a = iter::once(0).chain(binding.split(' ').map(|x| x.parse::<i64>().unwrap()));
        let begin_pos = (n as f32 / 2.).ceil() as usize - 1;
        println!(
            "{}",
            a.skip(begin_pos * k + 1)
                .step_by(n - begin_pos)
                .sum::<i64>()
        );
    }
}
