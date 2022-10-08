use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
#[allow(dead_code)]
fn read_3_number() -> (usize, usize, usize) {
    let binding = readline();
    let mut it = binding.split(' ').into_iter().map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}
fn read_2_number() -> (usize, usize) {
    let binding = readline();
    let mut it = binding.split(' ').map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}
#[allow(non_snake_case)]
fn main() {
    let (H, W) = read_2_number();
    let mut G: Vec<Vec<char>> = vec![vec![' ']; 520];
    let mut vis: Vec<Vec<bool>> = vec![vec![false; W as usize + 1]; H as usize + 1];
    for i in 1..=H {
        G[i as usize].extend(readline().chars());
    }
    let mut i = 1;
    let mut j = 1;
    loop {
        match G[i as usize][j as usize] {
            'U' => {
                if i != 1 {
                    i -= 1
                } else {
                    println!("{} {}", i, j);
                    break;
                }
            }
            'D' => {
                if i != H {
                    i += 1;
                } else {
                    println!("{} {}", i, j);
                    break;
                }
            }
            'L' => {
                if j != 1 {
                    j -= 1
                } else {
                    println!("{} {}", i, j);
                    break;
                }
            }
            'R' => {
                if j != W {
                    j += 1
                } else {
                    println!("{} {}", i, j);
                    break;
                }
            }
            _ => {}
        }
        if vis[i][j] {
            println!("-1");
            break;
        }
        vis[i][j] = true;
    }
}
