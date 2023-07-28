// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     let v: Vec<i32> = buf.split(' ')
//         .map(|x| x.chars().rev().collect::<String>())
//         .map(|x| x.parse().unwrap()).collect();
//     println!("{}", v[0].max(v[1]));
// }

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let m: i32 = buf.split_whitespace()
        .map(|x| x.chars().rev().collect::<String>())
        .map(|x| x.parse().unwrap()).max().unwrap();
    println!("{m}");
}