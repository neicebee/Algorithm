use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut n = buf.split_ascii_whitespace().flat_map( str::parse::<i32> );
    let (_, v) = (n.next(), n.next_back().unwrap());
    let cnt = n.filter(|&x| x==v).count();
    println!("{cnt}");
}