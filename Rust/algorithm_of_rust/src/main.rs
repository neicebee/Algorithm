// use std::io::{self, Read};

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     for l in buf.lines().skip(1) {
//         let s = l.chars().map(|x| x.to_digit(10).unwrap())
//             .sum::<u32>();
//         println!("{s}");
//     }
// }

use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    for l in buf.lines().skip(1) {
        let s = l.chars().map(|x| x.to_digit(10).unwrap())
            .sum::<u32>();
        println!("{s}");
    }
}