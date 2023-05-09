use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
        .unwrap();
    buf.pop();
    let n = buf.parse::<i32>().unwrap();
    
    for i in 1..n {
        println!("{i}");
    }

    for i in 1..=n {
        println!("{i}");
    }
}