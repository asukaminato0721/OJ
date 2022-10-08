use std::io;
fn readline() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().into()
}

fn main() {
    println!("{}", format!("{:01$X}", readline().parse::<u8>().unwrap(), 2));
}
