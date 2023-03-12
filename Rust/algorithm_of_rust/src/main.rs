use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let tmp: u16 = buf.trim().parse().unwrap();
    // let score = if 90<=tmp<=100 {
    //     'A'
    // } else if 80<=tmp<=89 {
    //     'B'
    // } else if 70<=tmp<=79 {
    //     'C'
    // } else if 60<=tmp<=69 {
    //     'D'
    // } else {
    //     'F'
    // };
    println!("{tmp}");
}